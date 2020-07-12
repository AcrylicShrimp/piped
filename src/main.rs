mod core;

use std::fs::read_to_string;

fn main() {
    let mut lexer = core::lexer::Lexer::new(read_to_string("examples/basics.piped").unwrap());

    loop {
        let token = lexer.next();

        if token.token_type == core::lexer::TokenType::Eof {
            break;
        }

        println!("{:#?}", token);
    }
}
