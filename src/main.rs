mod lexer;
mod repl;
mod ast;
mod parser;

use repl::*;

fn main() {
    println!("Welcome to the Monkey repl!");
    start();
}
