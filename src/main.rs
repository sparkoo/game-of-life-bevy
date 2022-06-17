use bevy::{core::FixedTimestep, prelude::*};
use rand::prelude::*;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game of Life!".to_string(),
            width: 800.0,
            height: 800.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_cells)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
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

fn step(mut cells: Query<(&mut Cell, &mut Sprite)>) {
    for (mut cell, mut sprite) in cells.iter_mut() {
        cell.state = match cell.state {
            CellState::Alive => CellState::Dead,
            CellState::Dead => CellState::Alive,
        };
        sprite.color = cell.state.color();
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

const CELL_ALIVE_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const CELL_DEAD_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

#[derive(Component)]
struct Cell {
    state: CellState,
}

#[derive(Component)]
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
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let cell_state = match rand::thread_rng().gen_bool(0.5) {
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
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / WIDTH as f32 * window.width() as f32,
            sprite_size.height / HEIGHT as f32 * window.height() as f32,
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
            convert(pos.x as f32, window.width() as f32, WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, HEIGHT as f32),
            0.0,
        );
    }
}
