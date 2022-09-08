use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::director::{BackwardsEvent, ForwardEvent};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(mouse_input)
            .add_system(mouse_scroll)
            .add_system(keyboard_input);
    }
}

fn mouse_input(mut ev_forward: EventWriter<ForwardEvent>, buttons: Res<Input<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Left) {
        ev_forward.send(ForwardEvent);
    }
}

fn mouse_scroll(
    mut ev_scroll: EventReader<MouseWheel>,
    mut ev_forward: EventWriter<ForwardEvent>,
    mut ev_backwards: EventWriter<BackwardsEvent>,
) {
    for ev in ev_scroll.iter() {
        if ev.y < 0. {
            ev_forward.send(ForwardEvent);
        } else if ev.y > 0. {
            ev_backwards.send(BackwardsEvent);
        }
    }
}

fn keyboard_input(mut ev_forward: EventWriter<ForwardEvent>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        ev_forward.send(ForwardEvent);
    }
}
