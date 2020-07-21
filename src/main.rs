mod compiler;
mod runtime;

use clap::{App, Arg};
use compiler::lookahead_lexer::LookaheadLexer as Lexer;
use compiler::parser::parse;
use runtime::context::Context;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let matches = App::new("piped")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A pipeline orchestrator for everyone to build softwares easier and faster.")
        .arg(
            Arg::with_name("input")
                .help("A pipeline scripe file to execute")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input = matches.values_of("input").unwrap().last().unwrap();

    let mut lexer = Lexer::new(match read_to_string(input) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Unable to open the given path: {}", input);
            eprintln!("\tbecause: {}", err);
            eprintln!("\texiting.");
            exit(-1);
        }
    });
    let ast_vec = parse(&mut lexer);
    let mut context = Context::new();

    context.execute(&ast_vec);
}
