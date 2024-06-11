pub struct Lexer<'a> {
    pub text: &'a str,
    cursor_index: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Assign,
    Equals,
    NotEqual,
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

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub value: &'a str,
}

impl<'z> Lexer<'z> {
    pub fn new(text: &'z str) -> Self {
        Lexer {
            text,
            cursor_index: 0,
        }
    }
    fn next_char(&mut self) -> Option<char> {
        let text_slice = &self.text[self.cursor_index..];
        let mut chars = text_slice.chars();
        self.cursor_index += 1;
        chars.next()
    }

    fn peek(&mut self) -> Option<char> {
        let text_slice = &self.text[self.cursor_index..];
        let mut chars = text_slice.chars();
        chars.next()
    }

    pub fn next_token<'a>(&mut self, text: &'a str) -> Token<'a> {
        let mut current = match self.next_char() {
            Some(v) => v,
            None => {
                return Token {
                    token_type: TokenType::EOF,
                    value: "",
                }
            }
        };

        while current.is_whitespace() {
            current = match self.next_char() {
                Some(v) => v,
                None => {
                    return Token {
                        token_type: TokenType::EOF,
                        value: "",
                    }
                }
            }
        }
        let slice_start = self.cursor_index - 1;

        if current == '"' {
            current = match self.next_char() {
                Some(v) => v,
                None => {
                    return Token {
                        token_type: TokenType::Unknown,
                        value: "",
                    }
                }
            };

            while current != '"' {
                current = match self.next_char() {
                    Some(v) => v,
                    None => {
                        return Token {
                            token_type: TokenType::Unknown,
                            value: "",
                        }
                    }
                }
            }
            return Token {
                token_type: TokenType::String,
                value: &text[slice_start..self.cursor_index],
            };
        }

        if current.is_alphabetic() {
            while self.peek().is_some_and(|c| c.is_alphabetic()) {
                self.next_char();
                let current_string = &text[slice_start..self.cursor_index];
                if current_string.chars().last().unwrap_or_else(|| '\0') == '"'
                    && current_string.chars().next().unwrap_or_else(|| '\0') == '"'
                {
                    return Token {
                        token_type: TokenType::String,
                        value: current_string,
                    };
                }
                if current_string == "LET" {
                    if !self.peek().is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::LET,
                            value: "LET",
                        };
                    }
                }
                if current_string == "PRINT" {
                    if !self.peek().is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::PRINT,
                            value: "PRINT",
                        };
                    }
                }
                if current_string == "IF" {
                    if !self.peek().is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::IF,
                            value: "IF",
                        };
                    }
                }
                if current_string == "THEN" {
                    if !self.peek().is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::THEN,
                            value: "THEN",
                        };
                    }
                }
                if current_string == "ENDIF" {
                    if !self.peek().is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::ENDIF,
                            value: "ENDIF",
                        };
                    }
                }
                if current_string == "WHILE" {
                    if !self.peek().is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::WHILE,
                            value: "WHILE",
                        };
                    }
                }
                if current_string == "REPEAT" {
                    if !self.peek().is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::REPEAT,
                            value: "REPEAT",
                        };
                    }
                }
                if current_string == "ENDWHILE" {
                    if !self.peek().is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::ENDWHILE,
                            value: "ENDWHILE",
                        };
                    }
                }
                if current_string == "INPUT" {
                    if !self.peek().is_some_and(|c| c.is_alphabetic()) {
                        return Token {
                            token_type: TokenType::INPUT,
                            value: "INPUT",
                        };
                    }
                }
            }
            return Token {
                token_type: TokenType::Ident,
                value: &text[slice_start..self.cursor_index],
            };
        }

        if current.is_digit(10) {
            while self.peek().is_some_and(|c| c.is_digit(10)) {
                self.next_char();
            }
            return Token {
                token_type: TokenType::Number,
                value: &text[slice_start..self.cursor_index],
            };
        }

        if current == '=' {
            if self.peek().is_some_and(|c| c == '=') {
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
            let next = self.peek();
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
            let next = self.peek();
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
