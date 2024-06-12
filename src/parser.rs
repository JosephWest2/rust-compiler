use std::collections::HashSet;

use crate::emitter::Emitter;
use crate::lexer::{Lexer, Token, TokenType};

pub struct Parser<'a> {
    current_token: Token<'a>,
    previous_token: Token<'a>,
    lexer: Lexer<'a>,
    emitter: Emitter,
    declared_idents: HashSet<&'a str>,
}

impl<'z> Parser<'z> {
    pub fn new(text: &'z str) -> Self {
        Parser {
            current_token: Token {
                token_type: TokenType::Unknown,
                value: "",
            },
            previous_token: Token {
                token_type: TokenType::Unknown,
                value: "",
            },
            lexer: Lexer::new(text),
            emitter: Emitter::new(),
            declared_idents: HashSet::new(),
        }
    }
    fn next_token(&mut self) {
        println!("{:?}", self.current_token);
        self.previous_token = self.current_token.clone();
        self.current_token = self.lexer.next_token(self.lexer.text);
    }
    fn emit_current_value_and_cycle(&mut self) {
        self.emitter.emit_to_buffer(self.current_token.value);
        self.next_token();
    }
    fn emit_text_and_cycle(&mut self, emit_text: &str) {
        self.emitter.emit_to_buffer(emit_text);
        self.next_token();
    }
    pub fn run(&mut self) {
        self.program();
    }
    fn program(&mut self) {
        self.emitter.emit_to_buffer("#include <stdio.h>\n");
        self.emitter.emit_to_buffer("int main(void) {\n");
        self.next_token();

        if !self.token_begins_statement() {
            eprintln!("expected expression start got {:?}", self.current_token);
            panic!();
        }
        while self.token_begins_statement() {
            self.statement();
        }

        self.emitter.emit_to_buffer("}\n");
        let res = self.emitter.write_buffer_to_file("OUTPUT.c");
        match res {
            Ok(_) => println!("file OUTPUT.c created successfully"),
            Err(e) => eprintln!("Error {} creating output file", e),
        }
    }
    fn token_begins_statement(&mut self) -> bool {
        match self.current_token.token_type {
            TokenType::PRINT => true,
            TokenType::IF => true,
            TokenType::WHILE => true,
            TokenType::LET => true,
            TokenType::INPUT => true,
            TokenType::Ident => {
                if self.declared_idents.contains(self.current_token.value) {
                    return true;
                }
                eprintln!("ident {} used before declaration", self.current_token.value);
                panic!();
            }
            _ => false,
        }
    }
    fn statement(&mut self) {
        match self.current_token.token_type {
            TokenType::PRINT => {
                self.emit_text_and_cycle("printf(");
                match self.current_token.token_type {
                    TokenType::String => self.emit_current_value_and_cycle(),
                    _ => {
                        self.emitter.emit_to_buffer("\"%f\", ");
                        self.expression();
                    }
                }
                self.emitter.emit_to_buffer(")");
                self.newline();
            }
            TokenType::IF => {
                self.emit_text_and_cycle("if (");
                self.comparison();
                self.emitter.emit_to_buffer(")");
                match self.current_token.token_type {
                    TokenType::THEN => self.emit_text_and_cycle("{"),
                    _ => {
                        eprintln!("expected THEN got {:?}", self.current_token);
                        panic!();
                    }
                }
                self.newline();
                while self.token_begins_statement() {
                    self.statement();
                }
                match self.current_token.token_type {
                    TokenType::ENDIF => self.emit_text_and_cycle("}"),
                    _ => {
                        eprintln!("expected ENDIF got {:?}", self.current_token);
                        panic!();
                    }
                }
                self.newline();
            }
            TokenType::WHILE => {
                self.emit_text_and_cycle("while (");
                self.comparison();
                self.emitter.emit_to_buffer(")");
                match self.current_token.token_type {
                    TokenType::REPEAT => self.emit_text_and_cycle("{"),
                    _ => {
                        eprintln!("expected REPEAT got {:?}", self.current_token);
                        panic!();
                    }
                }
                self.newline();
                while self.token_begins_statement() {
                    self.statement();
                }
                match self.current_token.token_type {
                    TokenType::ENDWHILE => self.emit_text_and_cycle("}"),
                    _ => {
                        eprintln!("expected ENDWHILE got {:?}", self.current_token);
                        panic!();
                    }
                }
                self.newline();
            }
            TokenType::LET => {
                self.emit_text_and_cycle("float ");
                match self.current_token.token_type {
                    TokenType::Ident => {
                        self.declared_idents.insert(self.current_token.value);
                        self.emit_current_value_and_cycle();
                    },
                    _ => {
                        eprintln!("expected Ident got {:?}", self.current_token);
                        panic!();
                    }
                }
                match self.current_token.token_type {
                    TokenType::Assign => self.emit_current_value_and_cycle(),
                    _ => {
                        eprintln!("expected assignment operator got {:?}", self.current_token);
                        panic!();
                    }
                }
                self.expression();
                self.newline();
            }
            TokenType::INPUT => {
                self.emit_current_value_and_cycle();
                match self.current_token.token_type {
                    TokenType::Ident => self.emit_current_value_and_cycle(),
                    _ => {
                        eprintln!("expected Ident got {:?}", self.current_token);
                        panic!();
                    }
                }
                self.newline();
            }
            TokenType::Ident => {
                self.emit_current_value_and_cycle();
                match self.current_token.token_type {
                    TokenType::Assign => self.emit_current_value_and_cycle(),
                    _ => {
                        eprintln!("expected assignment operator got {:?}", self.current_token);
                        panic!();
                    }
                }
                self.expression();
                self.newline();
            }
            _ => {
                eprintln!("expected keyword got {:?}", self.current_token);
                panic!();
            }
        }
    }
    fn expression(&mut self) {
        self.term();
        while matches!(self.current_token.token_type, TokenType::Plus) || matches!(self.current_token.token_type, TokenType::Minus) {
            self.emit_current_value_and_cycle();
            self.term();
        }
    }
    fn comparison(&mut self) {
        self.expression();
        match self.current_token.token_type {
            TokenType::Equals => self.emit_current_value_and_cycle(),
            TokenType::NotEqual => self.emit_current_value_and_cycle(),
            TokenType::Greater => self.emit_current_value_and_cycle(),
            TokenType::GreaterEqual => self.emit_current_value_and_cycle(),
            TokenType::Less => self.emit_current_value_and_cycle(),
            TokenType::LessEqual => self.emit_current_value_and_cycle(),
            _ => return,
        }
        self.expression();
    }
    fn term(&mut self) {
        self.unary();
        match self.current_token.token_type {
            TokenType::Plus => self.emit_current_value_and_cycle(),
            TokenType::Minus => self.emit_current_value_and_cycle(),
            _ => return,
        }
        self.unary();
    }
    fn unary(&mut self) {
        match self.current_token.token_type {
            TokenType::Plus => self.emit_current_value_and_cycle(),
            TokenType::Minus => self.emit_current_value_and_cycle(),
            _ => (),
        }
        self.primary();
    }
    fn primary(&mut self) {
        match self.current_token.token_type {
            TokenType::Number => {
                self.emit_current_value_and_cycle();
                self.emitter.emit_to_buffer(".f");
            }
            TokenType::Ident => {
                if !self.declared_idents.contains(self.current_token.value) {
                    eprintln!("ident {} used before declaration", self.current_token.value);
                    panic!();
                }
                self.emit_current_value_and_cycle();
            }
            _ => {
                eprintln!("expected number or ident got {:?}", self.current_token);
                panic!();
            }
        }
    }
    fn newline(&mut self) {
        self.emitter.emit_to_buffer(";");
        match self.current_token.token_type {
            TokenType::Newline => self.emit_current_value_and_cycle(),
            _ => {
                eprintln!("expected newline got {:?}", self.current_token);
                panic!();
            }
        }
    }
}
