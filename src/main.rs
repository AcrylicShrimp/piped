mod compiler;
mod core;

use compiler::lexer::{Lexer, TokenType};
use std::fs::read_to_string;

fn main() {
    let mut lexer = Lexer::new(read_to_string("examples/basics.piped").unwrap());

    loop {
        let token = lexer.next();

        if token.token_type == TokenType::Eof {
            break;
        }

        println!("{:#?}", token);
    }
}
