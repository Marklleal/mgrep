use std::{env, process};

use mgrep::Config;

fn main() {
    // Set up the command config based on the given arguments collected from CLI
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Run the program based on the informations provided
    if let Err(e) = mgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
