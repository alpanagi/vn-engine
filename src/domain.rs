use bevy::prelude::*;
use bevy::reflect::TypeUuid;

pub struct GameState {
    pub unparsed_scripts: Vec<Handle<Script>>,
    pub commands: Vec<Command>,
}
impl Default for GameState {
    fn default() -> Self {
        GameState {
            unparsed_scripts: vec![],
            commands: vec![],
        }
    }
}

#[derive(TypeUuid)]
#[uuid = "243e3962-da47-4f5f-8e60-4eab0598dc6a"]
pub struct Script {
    pub content: String,
}

#[derive(Debug)]
pub enum Command {
    Text { text: String },
}
