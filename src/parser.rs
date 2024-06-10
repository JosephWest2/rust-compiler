use std::process;

use crate::emitter;
use crate::lexer;

pub struct Parser<'a, 'b> {
    emitter: &'a mut dyn emitter::Emit,
    lexer: &'b mut dyn lexer::Lex,
    current_token: lexer::Token<'b>,
}

impl Parser<'_, '_> {
    pub fn new<'a, 'b>(emitter: &'a mut dyn emitter::Emit, lexer: &'b mut dyn lexer::Lex) -> Parser<'a, 'b> {
        Parser {
            emitter,
            lexer,
            current_token: lexer::Token {
                token_type: lexer::TokenType::Unknown,
                value: "",
            }
        }
    }
    pub fn run(&mut self) {
        self.program();
    }
    fn next_token(&mut self) -> lexer::Token {
        self.lexer.next_token()
    }
    fn program(&mut self) {
        self.emitter.emit_to_buffer("#include <stdio.h>\n");
        self.emitter.emit_to_buffer("int main(void) {\n");

        self.statement();

    }
    fn statement(&mut self) {

        let token = self.next_token();
        match token.token_type {

            lexer::TokenType::PRINT => {
                let token = self.next_token();
                match token.token_type {
                    lexer::TokenType::String => self.emitter.emit_to_buffer(token.value),
                    _ => self.expression(),
                }
            }
            lexer::TokenType::IF => {
                todo!();
            }
            lexer::TokenType::THEN => {
                todo!();
            }
            lexer::TokenType::ENDIF => {
                todo!();
            }
            lexer::TokenType::WHILE => {
                todo!();
            }
            lexer::TokenType::REPEAT => {
                todo!();
            }
            lexer::TokenType::ENDWHILE => {
                todo!();
            }
            lexer::TokenType::LET => {
                todo!();
            }
            lexer::TokenType::INPUT => {
                todo!();
            }

            _ => {
                eprintln!("expected keyword got {}", token.value);
                process::exit(1);
            }


        }

    }
    fn expression(&mut self) {

    }
}
