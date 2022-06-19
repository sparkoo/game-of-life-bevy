use bevy::{core::FixedTimestep, prelude::*, app::AppExit};
use rand::prelude::*;

const SIZE: i32 = 100;
const SIZE_CNT: i32 = SIZE * SIZE;
const CELL_ALIVE_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const CELL_DEAD_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const BACKGROUND_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game of Life!".to_string(),
            width: 800.0,
            height: 800.0,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_cells)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(step),
        )
        .add_system(keyboard_controls)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_plugins(DefaultPlugins)
        .run();
}

fn step(mut cells: Query<(&mut Cell, &Position, &mut Sprite)>, playing: ResMut<Playing>) {
    if !playing.0 {
        return;
    }

    let mut old: Vec<Cell> = Vec::new();
    for (cell, _, _) in cells.iter() {
        old.push(cell.clone())
    }

    for (mut cell, position, mut sprite) in cells.iter_mut() {
        let mut alive_neighs = 0;

        for neigh_pos in position.neighbor_coords(SIZE).iter() {
            match neigh_pos.to_index(SIZE, SIZE_CNT) {
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
        sprite.color = cell.state.color();
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
struct Playing(bool);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Cell {
    state: CellState,
}

impl Cell {
    fn change(&mut self, new_state: CellState) {
        self.state = new_state;
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}

impl CellState {
    fn color(&self) -> Color {
        match *self {
            CellState::Alive => CELL_ALIVE_COLOR,
            CellState::Dead => CELL_DEAD_COLOR,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
struct Position {
    x: i32,
    y: i32,
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

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

#[derive(Component)]
struct Colorable;

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn spawn_cells(mut commands: Commands) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            let cell_state = match rand::thread_rng().gen_bool(0.1) {
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
                .insert(Size::square(0.9));
        }
    }

    commands.insert_resource(Playing(true));
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / SIZE as f32 * window.width() as f32,
            sprite_size.height / SIZE as f32 * window.height() as f32,
            1.0,
        )
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, SIZE as f32),
            convert(pos.y as f32, -window.height() as f32, SIZE as f32),
            0.0,
        );
    }
}

fn keyboard_controls(keyboard_input: Res<Input<KeyCode>>, mut playing: ResMut<Playing>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        playing.0 = !playing.0;
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
