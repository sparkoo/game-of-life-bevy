use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use controls::ControlsPlugin;
use game::GamePlugin;
use render::RenderPlugin;
use clickable::Clickable;

mod clickable;
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
        .add_plugin(Clickable)
        .add_plugins(DefaultPlugins)
        //        .add_plugin(LogDiagnosticsPlugin::default())
        //        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
