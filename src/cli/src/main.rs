use std::env;
use std::process;

use cli::cli;

fn main() {
    let config = cli::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = cli::run(config) {
        eprintln!("Application error:: {err}");
        process::exit(1);
    }
}
