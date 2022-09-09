#[derive(Clone, PartialEq)]
pub enum Token {
    Text(String),
    Symbol(String),
    NewLine,
}

enum ParserState {
    Start,
    String,
    Symbol,
}

pub fn parse(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut parser_state = ParserState::Start;
    let mut current_stack = "".to_string();

    for ch in text.chars() {
        parser_state = match parser_state {
            ParserState::Start => start_state(ch, &mut current_stack, &mut tokens),
            ParserState::String => string_state(ch, &mut current_stack, &mut tokens),
            ParserState::Symbol => symbol_state(ch, &mut current_stack, &mut tokens),
        }
    }

    if let Some(token) = tokens.last() {
        if *token != Token::NewLine {
            tokens.push(Token::NewLine);
        }
    }

    tokens
}

fn start_state(ch: char, current_stack: &mut String, tokens: &mut Vec<Token>) -> ParserState {
    match ch {
        '"' => {
            return ParserState::String;
        }
        '\n' => {
            tokens.push(Token::NewLine);
            return ParserState::Start;
        }
        _ => {
            if ch.is_whitespace() {
                return ParserState::Start;
            } else if ch.is_alphabetic() {
                current_stack.push(ch);
                return ParserState::Symbol;
            } else {
                panic!("Parser error");
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

fn symbol_state(ch: char, current_stack: &mut String, tokens: &mut Vec<Token>) -> ParserState {
    if ch.is_alphanumeric() || ch == '_' || ch == '.' {
        current_stack.push(ch);
        return ParserState::Symbol;
    } else if ch == '\n' {
        tokens.push(Token::Symbol(current_stack.clone()));
        tokens.push(Token::NewLine);
        current_stack.clear();
        return ParserState::Start;
    } else if ch.is_whitespace() {
        tokens.push(Token::Symbol(current_stack.clone()));
        current_stack.clear();
        return ParserState::Start;
    } else {
        panic!("Invalid character");
    }
}
