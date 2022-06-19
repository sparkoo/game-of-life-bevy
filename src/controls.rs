use crate::game::Playing;
use bevy::{app::AppExit, prelude::*};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(keyboard_controls).add_system(mouse_controls);
    }
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

fn mouse_controls(windows: Res<Windows>, mouse_input: Res<Input<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        if let Some(position) = window.cursor_position() {
            println!("cursor position {}", position);
        } else {
            println!("cursor outside of the screen!");
        }
    }
}