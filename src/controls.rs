use bevy::{app::AppExit, prelude::*};

use crate::{game::{Playing, ClickedCellEvent, StepTimer}, components::cell::Cell};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(keyboard_controls).add_system(mouse_controls);
    }
}

fn keyboard_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut play: ResMut<Playing>,
    mut exit: EventWriter<AppExit>,
    mut timer: ResMut<StepTimer>,
    mut cells: Query<&mut Cell>,
) {
    println!("input {:?}", keyboard_input);

    if keyboard_input.just_pressed(KeyCode::Space) {
        play.toggle();
    }

    if keyboard_input.just_pressed(KeyCode::R) {
        play.stop();
        for mut cell in cells.iter_mut() {
            cell.die();
        }
    }

    if keyboard_input.just_pressed(KeyCode::Escape) || keyboard_input.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }

    if keyboard_input.just_pressed(KeyCode::NumpadAdd) || keyboard_input.just_pressed(KeyCode::Equals) {
        timer.inc();
    }

    if keyboard_input.just_pressed(KeyCode::NumpadSubtract) || keyboard_input.just_pressed(KeyCode::Minus) {
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
