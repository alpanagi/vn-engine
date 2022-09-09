#[derive(Clone, PartialEq)]
pub enum Token {
    Text(String),
    NewLine,
}

enum ParserState {
    Start,
    String,
}

pub fn parse(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut parser_state = ParserState::Start;
    let mut current_stack = "".to_string();

    for ch in text.chars() {
        parser_state = match parser_state {
            ParserState::Start => start_state(ch, &mut current_stack, &mut tokens),
            ParserState::String => string_state(ch, &mut current_stack, &mut tokens),
        }
    }

    if let Some(token) = tokens.last() {
        if *token != Token::NewLine {
            tokens.push(Token::NewLine);
        }
    }

    tokens
}

fn start_state(ch: char, _current_stack: &mut String, tokens: &mut Vec<Token>) -> ParserState {
    match ch {
        '"' => {
            return ParserState::String;
        }
        '\n' => {
            tokens.push(Token::NewLine);
            return ParserState::Start;
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

fn string_state(ch: char, current_stack: &mut String, tokens: &mut Vec<Token>) -> ParserState {
    match ch {
        '"' => {
            tokens.push(Token::Text(current_stack.clone()));
            current_stack.clear();
            return ParserState::Start;
        }
        _ => {
            current_stack.push(ch);
            return ParserState::String;
        }
    }
}
