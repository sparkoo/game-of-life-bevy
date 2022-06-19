use bevy::{core::FixedTimestep, prelude::*};
use rand::prelude::*;

use crate::clickable::OnClickSprite;
use crate::components::cell::{Cell, CellState, Position};

use crate::components;
use crate::consts;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickedCellEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.1))
                    .with_system(step),
            )
            .add_system(clicked_on_cell)
            .add_startup_system(spawn_cells);
    }
}

fn spawn_cells(mut commands: Commands) {
    for y in 0..consts::SIZE {
        for x in 0..consts::SIZE {
            // start with all dead, because we want to set live cells with mouse or load from file
            let cell_state = match rand::thread_rng().gen_bool(0.0) {
                true => CellState::Alive,
                false => CellState::Dead,
            };

            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: cell_state.color(),
                        ..default()
                    },
                    ..default()
                })
                .insert(Cell { state: cell_state })
                .insert(Position {
                    x: x as i32,
                    y: y as i32,
                })
                .insert(crate::render::Size::square(0.9));
        }
    }

    commands.insert_resource(Playing(false));
}

#[derive(Component)]
pub struct Playing(bool);

impl Playing {
    pub fn toggle(&mut self) {
        self.0 = !self.0
    }
}

#[derive(Debug)]
pub struct ClickedCellEvent {
    pub x: f32,
    pub y: f32,
}

fn clicked_on_cell(mut ev_clicked: EventReader<OnClickSprite>, mut q: Query<&mut Cell>) {
    for ev in ev_clicked.iter() {
        if let Ok(mut c) = q.get_mut(ev.entity) {
            c.state = CellState::Alive;
        } else {
            eprintln!("failed to query");
        }
    }
}

impl Position {
    fn neighbor_coords(&self, side_size: i32) -> Vec<Position> {
        let mut neighbors: Vec<Position> = Vec::new();
        // above me
        if self.y > 0 {
            if self.x > 0 {
                neighbors.push(Position {
                    x: self.x - 1,
                    y: self.y - 1,
                });
            }
            neighbors.push(Position {
                x: self.x,
                y: self.y - 1,
            });
            if side_size - self.x % side_size != 1 {
                neighbors.push(Position {
                    x: self.x + 1,
                    y: self.y - 1,
                });
            }
        }

        // in line with me
        if self.x > 0 {
            neighbors.push(Position {
                x: self.x - 1,
                y: self.y,
            });
        }
        if side_size - self.x % side_size != 1 {
            neighbors.push(Position {
                x: self.x + 1,
                y: self.y,
            });
        }

        // below me
        if side_size - self.y % side_size != 1 {
            if self.x > 0 {
                neighbors.push(Position {
                    x: self.x - 1,
                    y: self.y + 1,
                });
            }
            neighbors.push(Position {
                x: self.x,
                y: self.y + 1,
            });
            if side_size - self.x % side_size != 1 {
                neighbors.push(Position {
                    x: self.x + 1,
                    y: self.y + 1,
                });
            }
        }
        neighbors
    }

    fn to_index(&self, side_size: i32, size: i32) -> Option<i32> {
        let i: i32 = (side_size * self.y) + self.x;
        if i < 0 || i >= size {
            None
        } else {
            Some(i)
        }
    }
}

fn step(mut cells: Query<(&mut Cell, &Position, &mut Sprite)>, playing: ResMut<Playing>) {
    let mut old: Vec<Cell> = Vec::new();
    for (cell, _, mut sprite) in cells.iter_mut() {
        sprite.color = cell.state.color();
        old.push(cell.clone())
    }

    if !playing.0 {
        return;
    }
    
    for (mut cell, position, _) in cells.iter_mut() {
        let mut alive_neighs = 0;

        for neigh_pos in position.neighbor_coords(consts::SIZE).iter() {
            match neigh_pos.to_index(consts::SIZE, consts::SIZE_CNT) {
                Some(neigh_i) => match old.get(neigh_i as usize) {
                    Some(neighbor_cell) => match neighbor_cell.state {
                        CellState::Alive => alive_neighs += 1,
                        CellState::Dead => {}
                    },
                    None => eprintln!(
                        "no neighbor found at index {}. This should not happen!",
                        neigh_i
                    ),
                },
                None => eprintln!(
                    "failed to convert position {:?} to index. This should not happen!",
                    neigh_pos
                ),
            }
        }
        match cell.state {
            CellState::Alive => match alive_neighs {
                1 | 4 => cell.change(CellState::Dead),
                _ => {}
            },
            CellState::Dead => match alive_neighs {
                3 => cell.change(CellState::Alive),
                _ => {}
            },
        };
    }
}
