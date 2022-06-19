use bevy::prelude::*;

pub struct Clickable;

#[derive(Debug)]
pub struct OnClickSprite {
    pub entity: Entity,
}

impl Plugin for Clickable {
    fn build(&self, app: &mut App) {
        app.add_event::<OnClickSprite>().add_system(on_click);
    }
}

fn on_click(
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    sprites: Query<(Entity, &Transform)>,
    mut clicked_event_writer: EventWriter<OnClickSprite>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        if let Some(mut pos) = window.cursor_position() {
            pos.x -= window.width() / 2.0;
            pos.y -= window.height() / 2.0;
            for (e, t) in sprites.iter() {
                if pos.x >= t.translation.x - t.scale.x
                    && pos.x <= t.translation.x + t.scale.x
                    && pos.y >= t.translation.y - t.scale.y
                    && pos.y <= t.translation.y + t.scale.y
                {
                    clicked_event_writer.send(OnClickSprite { entity: e });
                    println!("clicked? [{:?}] {:?} {:?}", pos, e, t)
                }
            }
        }
    }
}
