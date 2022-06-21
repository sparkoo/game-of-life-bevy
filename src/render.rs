use bevy::{prelude::*, window::PresentMode};

use crate::consts;
use crate::components::cell::Position;
use crate::game::{Playing, StepTimer};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Game of Life!".to_string(),
            width: 800.0,
            height: 800.0,
            resizable: false,
//            present_mode: PresentMode::Mailbox,
            ..default()
        })
        .add_system(update_title)
        .insert_resource(ClearColor(consts::BACKGROUND_COLOR))
        .add_startup_system(setup_camera)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        );
    }
}

struct Title {
    playing: bool,
    speed: f32,
}

fn update_title(mut windows: ResMut<Windows>, playing: Res<Playing>, timer: Res<StepTimer>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(format!("Game of Life! [{}] [{} steps per second]; [+ speed up] [- slow down] [R reset] [Q exit]", if playing.0 {"playing"} else {"stopped"}, timer.steps_per_second));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / consts::SIZE as f32 * window.width() as f32,
            sprite_size.height / consts::SIZE as f32 * window.height() as f32,
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
            convert(pos.x as f32, window.width() as f32, consts::SIZE as f32),
            convert(pos.y as f32, -window.height() as f32, consts::SIZE as f32),
            0.0,
        );
    }
}

#[derive(Component)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
