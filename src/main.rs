use std::env;
use std::process;

use minigrep::{Config, USAGE};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        eprintln!("{USAGE}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}


