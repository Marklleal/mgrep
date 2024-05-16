use std::{env, process};

use mgrep::Config;

fn main() {
    // Collect the arguments from the command line
    let args: Vec<String> = env::args().collect();

    // Set up the command config based on the given arguments
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Run the program based on the informations provided
    if let Err(e) = mgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
