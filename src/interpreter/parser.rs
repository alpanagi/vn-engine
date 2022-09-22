use std::collections::VecDeque;

use super::tokenizer::Token;

#[derive(Clone)]
pub enum Command {
    Text {
        speaker: Option<String>,
        text: String,
    },
    DisplayBackground {
        image_path: String,
    },
}

pub fn generate_commands(tokens: &Vec<Token>) -> VecDeque<Command> {
    let mut queue: &[Token] = tokens;
    let mut commands = VecDeque::new();

    while queue.len() > 0 {
        match &queue[..] {
            [Token::Text(speaker), Token::Text(text), Token::NewLine, ..] => {
                commands.push_back(Command::Text {
                    speaker: Some(speaker.clone()),
                    text: text.clone(),
                });
                queue = &queue[3..];
            }
            [Token::Text(text), Token::NewLine, ..] => {
                commands.push_back(Command::Text {
                    speaker: None,
                    text: text.to_owned(),
                });
                queue = &queue[2..];
            }
            [Token::Symbol(command), Token::Symbol(target), ..] => {
                if command == "background" {
                    commands.push_back(Command::DisplayBackground {
                        image_path: target.clone(),
                    })
                } else {
                    panic!("Syntax error");
                }
                queue = &queue[2..];
            }
            [Token::NewLine, ..] => queue = &queue[1..],
            [..] => {
                panic!("Syntax error");
            }
        }
    }

    commands
}
