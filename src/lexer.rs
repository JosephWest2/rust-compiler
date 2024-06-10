pub trait Lex {
    fn next_token<'a>(&'a mut self) -> Token<'a>;
}

pub struct Lexer<'a> {
    text: &'a str,
    cursor_index: usize,
}

#[derive(Debug)]
pub enum TokenType {
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
    String,
    Times,
    Divide,
    Newline,
    EOF,
    Unknown,

    // Keywords
    LET,
    PRINT,
    IF,
    THEN,
    ENDIF,
    WHILE,
    REPEAT,
    ENDWHILE,
    INPUT,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub value: &'a str,
}

impl Lex for Lexer<'_> {
    fn next_token<'a>(&'a mut self) -> Token<'a> {
        let cursor_index = &mut self.cursor_index;
        let text = &self.text;
        let mut current = match Self::next_char(text, cursor_index) {
            Some(v) => v,
            None => {
                return Token {
                    token_type: TokenType::EOF,
                    value: "",
                }
            }
        };

        while current.is_whitespace() {
            current = match Self::next_char(text, cursor_index) {
                Some(v) => v,
                None => {
                    return Token {
                        token_type: TokenType::EOF,
                        value: "",
                    }
                }
            }
        }
        let slice_start = *cursor_index - 1;

        if current.is_alphabetic() {
            while Self::peek(text, cursor_index).is_some_and(|c| c.is_alphabetic()) {
                Self::next_char(text, cursor_index);
                let current_string = &text[slice_start..*cursor_index];
                if current_string.chars().last().unwrap_or_else(|| '\0') == '"' && current_string.chars().next().unwrap_or_else(|| '\0') == '"' {
                    return Token {
                        token_type: TokenType::String,
                        value: current_string,
                    }
                }
                if current_string == "LET" {
                    if !Self::peek(text, cursor_index).is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::LET,
                            value: "LET",
                        };
                    }
                }
                if current_string == "PRINT" {
                    if !Self::peek(text, cursor_index).is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::PRINT,
                            value: "PRINT",
                        };
                    }
                }
                if current_string == "IF" {
                    if !Self::peek(text, cursor_index).is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::IF,
                            value: "IF",
                        };
                    }
                }
                if current_string == "THEN" {
                    if !Self::peek(text, cursor_index).is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::THEN,
                            value: "THEN",
                        };
                    }
                }
                if current_string == "ENDIF" {
                    if !Self::peek(text, cursor_index).is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::ENDIF,
                            value: "ENDIF",
                        };
                    }
                }
                if current_string == "WHILE" {
                    if !Self::peek(text, cursor_index).is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::WHILE,
                            value: "WHILE",
                        };
                    }
                }
                if current_string == "REPEAT" {
                    if !Self::peek(text, cursor_index).is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::REPEAT,
                            value: "REPEAT",
                        };
                    }
                }
                if current_string == "ENDWHILE" {
                    if !Self::peek(text, cursor_index).is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::ENDWHILE,
                            value: "ENDWHILE",
                        };
                    }
                }
                if current_string == "INPUT" {
                    if !Self::peek(text, cursor_index).is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::INPUT,
                            value: "INPUT",
                        };
                    }
                }
            }
            return Token {
                token_type: TokenType::Ident,
                value: &text[slice_start..*cursor_index],
            };
        }

        if current.is_digit(10) {
            while Self::peek(text, cursor_index).is_some_and(|c| c.is_digit(10)) {
                Self::next_char(text, cursor_index);
            }
            return Token {
                token_type: TokenType::Number,
                value: &text[slice_start..*cursor_index],
            };
        }

        if current == '=' {
            if Self::peek(text, cursor_index).is_some_and(|c| c == '=') {
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
            let next = Self::peek(text, cursor_index);
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
            let next = Self::peek(text, cursor_index);
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

        if current == '\n' {
            return Token {
                token_type: TokenType::Newline,
                value: "\n",
            };
        }

        Token {
            token_type: TokenType::Unknown,
            value: "",
        }
    }
}

impl Lexer<'_> {
    pub fn new(file_text: &str) -> Lexer<'_> {
        Lexer {
            text: file_text,
            cursor_index: 0,
        }
    }
    fn next_char(text: &str, cursor_index: &mut usize) -> Option<char> {
        let text_slice = &text[*cursor_index..];
        let mut chars = text_slice.chars();
        *cursor_index += 1;
        chars.next()
    }

    fn peek(text: &str, cursor_index: &mut usize) -> Option<char> {
        let text_slice = &text[*cursor_index..];
        let mut chars = text_slice.chars();
        chars.next()
    }
}
