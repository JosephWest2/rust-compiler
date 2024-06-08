#[derive(Debug)]
enum TokenType {
    If,
    Then,
    Assign,
    Equals,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    Minus,
    Number,
    Ident,
    Times,
    Divide,
    Let,
    Print,
    Semicolon,
    EOF,
    Unknown,
}

#[derive(Debug)]
pub struct Token<'a> {
    token_type: TokenType,
    value: &'a str,
}

fn next_char(text: &String, text_start: &mut usize) -> Option<char> {
    let text_slice = &text[*text_start..];
    let mut chars = text_slice.chars();
    *text_start += 1;
    chars.next()
}

fn peek(text: &String, text_start: &mut usize) -> Option<char> {
    let text_slice = &text[*text_start..];
    let mut chars = text_slice.chars();
    chars.next()
}

fn next_token<'a>(text: &'a String, string_start: &mut usize) -> Token<'a> {
    let mut current = match next_char(text, string_start) {
        Some(v) => v,
        None => {
            return Token {
                token_type: TokenType::EOF,
                value: "",
            }
        }
    };

    while current.is_whitespace() {
        current = match next_char(text, string_start) {
            Some(v) => v,
            None => {
                return Token {
                    token_type: TokenType::EOF,
                    value: "",
                }
            }
        }
    }
    let slice_start = *string_start - 1;

    if current.is_alphabetic() {
        while peek(text, string_start).is_some_and(|c| c.is_alphabetic()) {
            next_char(text, string_start);
            if &text[slice_start..*string_start] == "LET" {
                if !peek(text, string_start).is_some_and(|c| c.is_alphabetic()) {
                    return Token {
                        token_type: TokenType::Let,
                        value: "LET",
                    };
                }
            }
            if &text[slice_start..*string_start] == "PRINT" {
                if !peek(text, string_start).is_some_and(|c| c.is_alphabetic()) {
                    return Token {
                        token_type: TokenType::Print,
                        value: "PRINT",
                    };
                }
            }
            if &text[slice_start..*string_start] == "IF" {
                if !peek(text, string_start).is_some_and(|c| c.is_alphabetic()) {
                    return Token {
                        token_type: TokenType::If,
                        value: "IF",
                    };
                }
            }
            if &text[slice_start..*string_start] == "THEN" {
                if !peek(text, string_start).is_some_and(|c| c.is_alphabetic()) {
                    return Token {
                        token_type: TokenType::Then,
                        value: "THEN",
                    };
                }
            }
        }
        return Token {
            token_type: TokenType::Ident,
            value: &text[slice_start..*string_start],
        };
    }

    if current.is_digit(10) {
        while peek(text, string_start).is_some_and(|c| c.is_digit(10)) {
            next_char(text, string_start);
        }
        return Token {
            token_type: TokenType::Number,
            value: &text[slice_start..*string_start],
        };
    }

    if current == '=' {
        if peek(text, string_start).is_some_and(|c| c == '=') {
            return Token {
                token_type: TokenType::Equals,
                value: "==",
            };
        }
        return Token {
            token_type: TokenType::Assign,
            value: "=",
        };
    }

    if current == '>' {
        let next = peek(text, string_start);
        if next.is_some_and(|c| c == '=') {
            return Token {
                token_type: TokenType::GreaterEqual,
                value: ">=",
            };
        }
        return Token {
            token_type: TokenType::Greater,
            value: ">",
        };
    }

    if current == '<' {
        let next = peek(text, string_start);
        if next.is_some_and(|c| c == '=') {
            return Token {
                token_type: TokenType::LessEqual,
                value: "<=",
            };
        }
        return Token {
            token_type: TokenType::Less,
            value: ">",
        };
    }

    if current == '+' {
        return Token {
            token_type: TokenType::Plus,
            value: "+",
        };
    }

    if current == '-' {
        return Token {
            token_type: TokenType::Minus,
            value: "-",
        };
    }

    if current == '*' {
        return Token {
            token_type: TokenType::Times,
            value: "*",
        };
    }

    if current == '/' {
        return Token {
            token_type: TokenType::Divide,
            value: "/",
        };
    }

    if current == ';' {
        return Token {
            token_type: TokenType::Semicolon,
            value: ";",
        };
    }

    Token {
        token_type: TokenType::Unknown,
        value: "",
    }
}


pub fn parse_string(text: &String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut i: usize = 0;
    loop {
        let next_token = next_token(text, &mut i);
        tokens.push(next_token);
        if tokens.last().is_some_and(|t| {
            matches!(t.token_type, TokenType::EOF) || matches!(t.token_type, TokenType::Unknown)
        }) {
            break;
        }
    }
    tokens
}
