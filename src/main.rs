use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use game::GamePlugin;
use render::RenderPlugin;
use controls::ControlsPlugin;

mod components;
mod consts;
mod controls;
mod game;
mod render;

fn main() {
    App::new()
        .add_plugin(GamePlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(ControlsPlugin)
        .add_plugins(DefaultPlugins)
//        .add_plugin(LogDiagnosticsPlugin::default())
//        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
