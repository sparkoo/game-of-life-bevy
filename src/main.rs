use bevy::{core::FixedTimestep, prelude::*};
use rand::prelude::*;

const SIZE: i32 = 4;
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
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(step),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_plugins(DefaultPlugins)
        .run();
}

fn step(mut cells: Query<(&mut Cell, &mut Sprite)>, mut playing: ResMut<Playing>) {
    if !playing.0 {
        return;
    }
    playing.0 = false;

    let mut old: Vec<bool> = Vec::new();
    for (cell, _) in cells.iter() {
        old.push(match cell.state {
            CellState::Alive => true,
            CellState::Dead => false,
        })
    }

    for (i, (mut cell, mut sprite)) in cells.iter_mut().enumerate() {
        let mut neigh = 0;

        println!("{} is {:?} and has {} neighs", i, cell.state,neigh);

        match cell.state {
            CellState::Alive => {
                match neigh {
                    1 | 4 => cell.change(CellState::Dead),
                    _ => {},
                }
            },
            CellState::Dead => {
                match neigh {
                    3 => cell.change(CellState::Alive),
                    _ => {},
                }

            }
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

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
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
            let cell_state = match rand::thread_rng().gen_bool(0.3) {
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
