use crate::emitter;
use crate::lexer;

struct Parser<'a, 'b> {
    emitter: &'a mut dyn emitter::Emit,
    lexer: &'b mut dyn lexer::Lex,
}

impl Parser<'_, '_> {
    fn new<'a, 'b>(emitter: &'a mut dyn emitter::Emit, lexer: &'b mut dyn lexer::Lex) -> Parser<'a, 'b> {
        Parser {
            emitter,
            lexer,
        }
    }
    fn program(&mut self) {

    }
    fn statement(&mut self) {

    }
    fn expression(&mut self) {

    }
}
