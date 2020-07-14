mod compiler;
mod core;

use compiler::lexer::Lexer;
use compiler::parser::parse;
use std::fs::read_to_string;

fn main() {
    let mut lexer = Lexer::new(read_to_string("examples/basics.piped").unwrap());
    let ast_vec = parse(&mut lexer);

    println!("{:#?}", ast_vec);
}
