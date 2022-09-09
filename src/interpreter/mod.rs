mod syntax_processor;
mod tokenizer;

use crate::domain::Command;

pub fn process(text: &String) -> Vec<Command> {
    let tokens = tokenizer::parse(text);
    let commands = syntax_processor::parse_commands(tokens);
    return commands;
}
