use crate::lexer::{Token, Lexer};
use crate::ast::*;


pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token
}


impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        Self {
            lexer,
            cur_token: lexer.next_token(),
            peek_token: lexer.next_token()
        }
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&self) -> Option<Program> {
        return None;
    }
}

