mod compiler;
mod runtime;

use clap::{App, Arg};
use runtime::execution::Execution;
use runtime::imported_pipeline::ImportedPipeline;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let matches = App::new("piped")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("input")
                .help("A pipeline scripe file to execute")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input = matches.values_of("input").unwrap().last().unwrap();
    let entry_pipeline = match ImportedPipeline::import(&PathBuf::from(input)) {
        Ok(pipeline) => pipeline,
        Err(err) => {
            eprintln!("Unable to read the given path: {}", input);
            eprintln!("\tbecause: {}", err);
            eprintln!("\texiting.");
            exit(-1);
        }
    };

    Execution::new().execute(entry_pipeline);
}
