use bevy::prelude::*;
use bevy::reflect::TypeUuid;

pub struct GameState {
    pub unprocessed_scripts: Vec<Handle<Script>>,
    pub commands: Vec<Command>,
    pub current_command: usize,
}
impl Default for GameState {
    fn default() -> Self {
        GameState {
            unprocessed_scripts: vec![],
            commands: vec![],
            current_command: 0,
        }
    }
}

#[derive(TypeUuid)]
#[uuid = "243e3962-da47-4f5f-8e60-4eab0598dc6a"]
pub struct Script {
    pub content: String,
}

#[derive(Clone)]
pub enum Command {
    NoOp,
    Text {
        speaker: Option<String>,
        text: String,
    },
    DisplayBackground {
        image: String,
    },
}
