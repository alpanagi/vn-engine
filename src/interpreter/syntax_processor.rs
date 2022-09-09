use crate::domain::Command;

use super::tokenizer::Token;

pub fn parse_commands(tokens: Vec<Token>) -> Vec<Command> {
    let mut queue: Vec<Token> = tokens.clone();
    let mut commands = vec![];

    while queue.len() > 0 {
        match &queue[..] {
            [Token::Text(speaker), Token::Text(text), Token::NewLine, ..] => {
                commands.push(Command::Text {
                    speaker: Some(speaker.clone()),
                    text: text.clone(),
                });
                queue = queue[3..].to_vec()
            }
            [Token::Text(text), Token::NewLine, ..] => {
                commands.push(Command::Text {
                    speaker: None,
                    text: text.to_owned(),
                });
                queue = queue[2..].to_vec()
            }
            [Token::NewLine, ..] => queue = queue[1..].to_vec(),
            [..] => {
                panic!("Syntax error");
            }
        }
    }

    commands
}
