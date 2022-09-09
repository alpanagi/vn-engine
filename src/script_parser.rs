use crate::domain::Command;

enum ParserState {
    Start,
    String,
}

pub fn parse(text: &str) -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];
    let mut parser_state = ParserState::Start;
    let mut current_stack = "".to_string();

    for ch in text.chars() {
        parser_state = match parser_state {
            ParserState::Start => start_state(ch, &mut current_stack, &mut commands),
            ParserState::String => string_state(ch, &mut current_stack, &mut commands),
        }
    }

    commands
}

fn start_state(ch: char, _current_stack: &mut String, _commands: &mut Vec<Command>) -> ParserState {
    match ch {
        '"' => {
            return ParserState::String;
        }
        _ => {
            if !ch.is_whitespace() {
                panic!("Parser error");
            } else {
                return ParserState::Start;
            }
        }
    }
}

fn string_state(ch: char, current_stack: &mut String, commands: &mut Vec<Command>) -> ParserState {
    match ch {
        '"' => {
            commands.push(Command::Text {
                text: current_stack.clone(),
            });
            current_stack.clear();
            return ParserState::Start;
        }
        _ => {
            current_stack.push(ch);
            return ParserState::String;
        }
    }
}
