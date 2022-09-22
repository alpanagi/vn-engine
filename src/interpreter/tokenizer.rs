enum State {
    Start,
    String,
    Symbol,
}

#[derive(Clone, PartialEq)]
pub enum Token {
    Text(String),
    Symbol(String),
    NewLine,
}

pub fn generate_tokens(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut parser_state = State::Start;
    let mut current_stack = String::new();

    for ch in text.chars() {
        parser_state = match parser_state {
            State::Start => start_state(ch, &mut current_stack, &mut tokens),
            State::String => string_state(ch, &mut current_stack, &mut tokens),
            State::Symbol => symbol_state(ch, &mut current_stack, &mut tokens),
        }
    }

    if let Some(token) = tokens.last() {
        if *token != Token::NewLine {
            tokens.push(Token::NewLine);
        }
    }

    tokens
}

fn start_state(ch: char, current_stack: &mut String, tokens: &mut Vec<Token>) -> State {
    match ch {
        '"' => {
            return State::String;
        }
        '\n' => {
            tokens.push(Token::NewLine);
            return State::Start;
        }
        _ => {
            if ch.is_whitespace() {
                return State::Start;
            } else if ch.is_alphabetic() {
                current_stack.push(ch);
                return State::Symbol;
            } else {
                panic!("Tokenizer error");
            }
        }
    }
}

fn string_state(ch: char, current_stack: &mut String, tokens: &mut Vec<Token>) -> State {
    match ch {
        '"' => {
            tokens.push(Token::Text(current_stack.clone()));
            current_stack.clear();
            return State::Start;
        }
        _ => {
            current_stack.push(ch);
            return State::String;
        }
    }
}

fn symbol_state(ch: char, current_stack: &mut String, tokens: &mut Vec<Token>) -> State {
    if ch.is_alphanumeric() || ch == '_' || ch == '.' {
        current_stack.push(ch);
        return State::Symbol;
    } else if ch == '\n' {
        tokens.push(Token::Symbol(current_stack.clone()));
        tokens.push(Token::NewLine);
        current_stack.clear();
        return State::Start;
    } else if ch.is_whitespace() {
        tokens.push(Token::Symbol(current_stack.clone()));
        current_stack.clear();
        return State::Start;
    } else {
        panic!("Tokenizer error");
    }
}
