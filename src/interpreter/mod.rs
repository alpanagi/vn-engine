mod parser;
mod tokenizer;

use std::collections::VecDeque;

pub use self::parser::Command;

pub fn parse_text(text: &String) -> VecDeque<Command> {
    let tokens = tokenizer::generate_tokens(text);
    let commands = parser::generate_commands(&tokens);
    return commands;
}
