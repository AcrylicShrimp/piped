mod compiler;
mod runtime;

use compiler::lookahead_lexer::LookaheadLexer as Lexer;
use compiler::parser::parse;
use runtime::context::Context;
use std::fs::read_to_string;

fn main() {
    let mut lexer = Lexer::new(read_to_string("examples/test.piped").unwrap());
    let ast_vec = parse(&mut lexer);
    let mut context = Context::new();

    context.execute(&ast_vec);
}
