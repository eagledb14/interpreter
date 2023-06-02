
use crate::lexer::Token;

pub trait Node {
    fn token_literal(&self) -> Token;
}


pub trait Statement {
    fn statement_node(&self);
}

pub trait NodeStatement: Node + Statement {}


pub trait Expression: Node {
    fn expression_node(&self);
}

pub trait NodeExpression: Node + Expression {}



pub struct Program {
    statements: Vec<Box<dyn NodeStatement>>
}

impl Node for Program {
    fn token_literal(&self) -> Token {
        match self.statements.first() {
            Some(val) => val.token_literal(),
            None => Token::Illegal
        }
    }
}


pub struct LetStatement {
    token: Token,
    value: Box<dyn NodeExpression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> Token {
        self.token.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {
        
    }
}


pub struct Identifier {
    token: Token,
    value: String
}

impl Node for Identifier {
    fn token_literal(&self) -> Token {
        self.token.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
        
    }
}






