mod parser;
mod tokenizer;

use std::collections::{HashMap, VecDeque};

use crate::domain::GameData;

pub use self::parser::Command;

pub fn parse_text(text: &String) -> (GameData, VecDeque<Command>) {
    let mut game_data: GameData = HashMap::new();
    let mut tokens = tokenizer::generate_tokens(text);
    let commands = parser::generate_commands(&mut tokens, &mut game_data);
    return (game_data, commands);
}
