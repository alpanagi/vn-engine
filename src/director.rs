use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{
    domain::{GameState, Screen},
    ui::{BackgroundDisplayEvent, TextDisplayEvent},
};

pub struct DirectorPlugin;
impl Plugin for DirectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(forward).add_system(backwards);
    }
}

fn forward(
    buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut ev_text_display: EventWriter<TextDisplayEvent>,
    mut ev_background_display: EventWriter<BackgroundDisplayEvent>,
) {
    let mut scrolled = false;
    for ev in ev_scroll.iter() {
        if ev.y < 0. {
            scrolled = true;
        }
    }

    if buttons.just_pressed(MouseButton::Left) || keys.just_pressed(KeyCode::Space) || scrolled {
        game_state.forward();
        if let Some(screen) = game_state.current_screen() {
            update_screen(&screen, &mut ev_text_display, &mut ev_background_display)
        }
    }
}

fn backwards(
    mut game_state: ResMut<GameState>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut ev_text_display: EventWriter<TextDisplayEvent>,
    mut ev_background_display: EventWriter<BackgroundDisplayEvent>,
) {
    for ev in ev_scroll.iter() {
        if ev.y > 0. {
            game_state.backwards();
            if let Some(screen) = game_state.current_screen() {
                update_screen(&screen, &mut ev_text_display, &mut ev_background_display)
            }
        }
    }
}

fn update_screen(
    screen: &Screen,
    ev_text_display: &mut EventWriter<TextDisplayEvent>,
    ev_background_display: &mut EventWriter<BackgroundDisplayEvent>,
) {
    ev_text_display.send(TextDisplayEvent {
        speaker: screen.speaker.clone(),
        speaker_color: screen.speaker_color,
        text: screen.text.clone(),
    });
    ev_background_display.send(BackgroundDisplayEvent {
        image_path: screen.background.clone(),
    });
}
