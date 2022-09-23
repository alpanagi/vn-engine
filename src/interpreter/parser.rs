use std::collections::{HashMap, VecDeque};

use crate::domain::GameData;

use super::tokenizer::{Token, TokenContainer};

#[derive(Clone)]
pub enum Command {
    Text {
        speaker: Option<String>,
        text: String,
    },
    CharacterText {
        speaker: String,
        text: String,
    },
    DisplayBackground {
        image_path: String,
    },
}

pub fn generate_commands(
    tokens: &Vec<TokenContainer>,
    game_data: &mut GameData,
) -> VecDeque<Command> {
    let mut queue: &[TokenContainer] = tokens;
    let mut commands = VecDeque::new();

    while queue.len() > 0 {
        match &queue[..] {
            [TokenContainer {
                token: Token::Text(speaker),
                ..
            }, TokenContainer {
                token: Token::Text(text),
                ..
            }, TokenContainer {
                token: Token::NewLine,
                ..
            }, ..] => {
                commands.push_back(Command::Text {
                    speaker: Some(speaker.clone()),
                    text: text.clone(),
                });
                queue = &queue[3..];
            }

            [TokenContainer {
                token: Token::Symbol(speaker),
                ..
            }, TokenContainer {
                token: Token::Text(text),
                ..
            }, TokenContainer {
                token: Token::NewLine,
                ..
            }, ..] => {
                commands.push_back(Command::CharacterText {
                    speaker: speaker.clone(),
                    text: text.clone(),
                });
                queue = &queue[3..];
            }

            [TokenContainer {
                token: Token::Text(text),
                ..
            }, TokenContainer {
                token: Token::NewLine,
                ..
            }, ..] => {
                commands.push_back(Command::Text {
                    speaker: None,
                    text: text.to_owned(),
                });
                queue = &queue[2..];
            }

            [TokenContainer {
                token: Token::Symbol(command),
                ..
            }, TokenContainer {
                token: Token::Symbol(target),
                ..
            }, ..] => {
                if command == "background" {
                    commands.push_back(Command::DisplayBackground {
                        image_path: target.clone(),
                    })
                } else {
                    panic!("Syntax error");
                }
                queue = &queue[2..];
            }

            [TokenContainer {
                token: Token::NewLine,
                ..
            }, ..] => queue = &queue[1..],

            [TokenContainer {
                token: Token::Symbol(name),
                ..
            }, TokenContainer {
                token: Token::Equals,
                ..
            }, TokenContainer {
                token: Token::ObjectStart,
                ..
            }, ..] => {
                let (new_queue, object) = parse_object(&queue[3..]);
                game_data.insert(name.clone(), object);
                queue = new_queue;
            }

            [..] => {
                if let Some(TokenContainer { location, .. }) = queue.first() {
                    panic!(
                        "Syntax error at location: (row: {}, col: {})",
                        location.row, location.col
                    );
                } else {
                    panic!("Syntax error");
                }
            }
        }
    }

    commands
}

fn parse_object(mut queue: &[TokenContainer]) -> (&[TokenContainer], HashMap<String, String>) {
    let mut map = HashMap::new();

    loop {
        if queue.len() == 0 {
            panic!("Finished parsing while creating object");
        }

        match &queue[..] {
            [TokenContainer {
                token: Token::ObjectEnd,
                ..
            }, ..] => {
                return (&queue[1..], map);
            }
            [TokenContainer {
                token: Token::Key(key),
                ..
            }, TokenContainer {
                token: Token::Symbol(val),
                ..
            }, ..] => {
                map.insert(key.clone(), val.clone());
                queue = &queue[2..];
            }
            [TokenContainer {
                token: Token::Key(key),
                ..
            }, TokenContainer {
                token: Token::Text(val),
                ..
            }, ..] => {
                map.insert(key.clone(), val.clone());
                queue = &queue[2..];
            }
            [TokenContainer {
                token: Token::NewLine,
                ..
            }, ..] => {
                queue = &queue[1..];
            }
            [TokenContainer {
                token: Token::Comma,
                ..
            }, ..] => {
                queue = &queue[1..];
            }
            _ => {
                if let Some(TokenContainer { location, .. }) = queue.first() {
                    panic!(
                        "Object creation error at location: (row: {}, col: {})",
                        location.row, location.col
                    );
                } else {
                    panic!("Object creation error");
                }
            }
        }
    }
}
