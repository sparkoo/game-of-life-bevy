use bevy::{app::AppExit, prelude::*};

use crate::game::{Playing, ClickedCellEvent, StepTimer};

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
    mut timer: ResMut<StepTimer>,
) {
//    println!("input {:?}", keyboard_input);

    if keyboard_input.just_pressed(KeyCode::Space) {
        playing.toggle()
    }

    if keyboard_input.just_pressed(KeyCode::Escape) || keyboard_input.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }

    if keyboard_input.just_pressed(KeyCode::NumpadAdd) {
        timer.inc();
    }

    if keyboard_input.just_pressed(KeyCode::NumpadSubtract) {
        timer.dec();
    }
}

fn mouse_controls(windows: Res<Windows>, mouse_input: Res<Input<MouseButton>>, mut clicked_event_writer: EventWriter<ClickedCellEvent>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        if let Some(position) = window.cursor_position() {
            clicked_event_writer.send(ClickedCellEvent { x: position.x, y: position.y })
        }
    }
}
