use crate::lexer::*;

use std::io::{self, Write};

pub fn start() {
    print!(">> ");
    io::stdout().flush().expect("");

    for input in io::stdin().lines() {
        let mut lexer = Lexer::new(input.unwrap());
        while let Ok(t) = lexer.next_token() {
            println!("{:?}", t);
            if t == Token::Eof {
                break;
            }
        }
        print!(">> ");
        io::stdout().flush().expect("");
    }
}
