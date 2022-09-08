use bevy::prelude::*;

use crate::{
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
    mut game_state: ResMut<GameState>,
) {
    for _ in ev_forward.iter() {
        if game_state.current_command < game_state.commands.len() - 1 {
            game_state.current_command += 1;
        }
    }

    for _ in ev_backwards.iter() {
        if game_state.current_command > 0 {
            game_state.current_command -= 1;
        }
    }

    if let Some(command) = game_state.commands.get(game_state.current_command) {
        match command {
            Command::Text { text } => ev_text.send(TextEvent { text: text.clone() }),
        }
    }
}
