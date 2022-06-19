use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{app::AppExit, prelude::*};
use game::*;

use render::*;

mod game;
mod render;
mod consts;

fn main() {
    App::new()
        .add_system(keyboard_controls)
        .add_plugin(RenderPlugin)
        .add_plugin(GamePlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn keyboard_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut playing: ResMut<Playing>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        playing.toggle()
    }

    if keyboard_input.just_pressed(KeyCode::Escape) || keyboard_input.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
}
