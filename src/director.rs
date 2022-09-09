use bevy::prelude::*;

use crate::{
    background::BackgroundEvent,
    domain::{Command, GameState},
    text::TextEvent,
};

pub struct ForwardEvent;
pub struct BackwardsEvent;

pub struct DirectorPlugin;
impl Plugin for DirectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ForwardEvent>()
            .add_event::<BackwardsEvent>()
            .add_system(director);
    }
}

fn director(
    mut ev_forward: EventReader<ForwardEvent>,
    mut ev_backwards: EventReader<BackwardsEvent>,
    mut ev_text: EventWriter<TextEvent>,
    mut ev_background: EventWriter<BackgroundEvent>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.commands.len() == 0 {
        return;
    }

    for _ in ev_forward.iter() {
        if game_state.current_command < game_state.commands.len() - 1 {
            game_state.current_command += 1;
            send_command(&mut ev_text, &mut ev_background, &game_state)
        }
    }

    for _ in ev_backwards.iter() {
        if game_state.current_command > 0 {
            game_state.current_command -= 1;
            send_command(&mut ev_text, &mut ev_background, &game_state)
        }
    }
}

fn send_command(
    ev_text: &mut EventWriter<TextEvent>,
    ev_background: &mut EventWriter<BackgroundEvent>,
    game_state: &ResMut<GameState>,
) {
    if let Some(command) = game_state.commands.get(game_state.current_command) {
        match command {
            Command::NoOp => {}
            Command::Text { speaker, text } => ev_text.send(TextEvent {
                speaker: speaker.clone(),
                text: text.clone(),
            }),
            Command::DisplayBackground { image } => ev_background.send(BackgroundEvent {
                image: image.clone(),
            }),
        }
    }
}
