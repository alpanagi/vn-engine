enum State {
    Start,
    String,
    Symbol,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Location {
    pub col: usize,
    pub row: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Text(String),
    Symbol(String),
    Key(String),
    NewLine,
    Equals,
    Comma,
    ObjectStart,
    ObjectEnd,
}

#[derive(Debug)]
pub struct TokenContainer {
    pub location: Location,
    pub token: Token,
}
impl TokenContainer {
    fn new(location: Location, token: Token) -> Self {
        TokenContainer { location, token }
    }
}

pub fn generate_tokens(text: &str) -> Vec<TokenContainer> {
    let mut tokens: Vec<TokenContainer> = vec![];
    let mut parser_state = State::Start;
    let mut current_stack = String::new();

    let mut location = Location { col: 1, row: 1 };

    for ch in text.chars() {
        if ch == '\n' {
            location.row += 1;
            location.col = 1;
        } else {
            location.col += 1;
        }

        parser_state = match parser_state {
            State::Start => start_state(ch, &mut location, &mut current_stack, &mut tokens),
            State::String => string_state(ch, &mut location, &mut current_stack, &mut tokens),
            State::Symbol => symbol_state(ch, &mut location, &mut current_stack, &mut tokens),
        }
    }

    if let Some(TokenContainer { token, .. }) = tokens.last() {
        if let Token::NewLine = token {
        } else {
            tokens.push(TokenContainer::new(location, Token::NewLine));
        }
    }

    tokens
}

fn start_state(
    ch: char,
    location: &mut Location,
    current_stack: &mut String,
    tokens: &mut Vec<TokenContainer>,
) -> State {
    match ch {
        '"' => {
            return State::String;
        }
        '\n' => {
            tokens.push(TokenContainer::new(*location, Token::NewLine));
            return State::Start;
        }
        '=' => {
            tokens.push(TokenContainer::new(*location, Token::Equals));
            return State::Start;
        }
        '{' => {
            tokens.push(TokenContainer::new(*location, Token::ObjectStart));
            return State::Start;
        }
        '}' => {
            tokens.push(TokenContainer::new(*location, Token::ObjectEnd));
            return State::Start;
        }
        ',' => {
            tokens.push(TokenContainer::new(*location, Token::Comma));
            return State::Start;
        }
        _ => {
            if ch.is_whitespace() {
                return State::Start;
            } else if ch.is_alphabetic() {
                current_stack.push(ch);
                return State::Symbol;
            } else {
                panic!(
                    "Tokenizer error at (row: {}, col: {})",
                    location.row, location.col
                );
            }
        }
    }
}

fn string_state(
    ch: char,
    location: &mut Location,
    current_stack: &mut String,
    tokens: &mut Vec<TokenContainer>,
) -> State {
    match ch {
        '"' => {
            tokens.push(TokenContainer::new(
                *location,
                Token::Text(current_stack.clone()),
            ));
            current_stack.clear();
            return State::Start;
        }
        _ => {
            current_stack.push(ch);
            return State::String;
        }
    }
}

fn symbol_state(
    ch: char,
    location: &mut Location,
    current_stack: &mut String,
    tokens: &mut Vec<TokenContainer>,
) -> State {
    if ch.is_alphanumeric() || ch == '_' || ch == '.' {
        current_stack.push(ch);
        return State::Symbol;
    } else if ch == '\n' {
        tokens.push(TokenContainer::new(
            *location,
            Token::Symbol(current_stack.clone()),
        ));
        tokens.push(TokenContainer::new(*location, Token::NewLine));
        current_stack.clear();
        return State::Start;
    } else if ch.is_whitespace() {
        tokens.push(TokenContainer::new(
            *location,
            Token::Symbol(current_stack.clone()),
        ));
        current_stack.clear();
        return State::Start;
    } else if ch == ':' {
        tokens.push(TokenContainer::new(
            *location,
            Token::Key(current_stack.clone()),
        ));
        current_stack.clear();
        return State::Start;
    } else if ch == ',' {
        tokens.push(TokenContainer::new(
            *location,
            Token::Symbol(current_stack.clone()),
        ));
        tokens.push(TokenContainer::new(*location, Token::Comma));
        current_stack.clear();
        return State::Start;
    } else {
        panic!(
            "Tokenizer error at location: (row: {}, col: {})",
            location.row, location.col
        );
    }
}
