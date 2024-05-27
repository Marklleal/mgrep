use std::{env, process};

use mgrep::Config;

/// Arguments should include the query, file path, and optionally, the option to ignore case.
/// Usage:
///     String Literal one Line:
///         echo "The literal string go here" | {ENVIRONMENT VARIABLE} cargo run -- ["QUERY"] [EXPRESSION]
///
///     String Literal Multiple Lines:
///         cat << EOF | {ENVIRONMET VARIABLE} cargo run -- ["QUERY"] [EXPRESSION]
///             The literal string
///             go
///             here
///         EOF
///
///     File Path:
///         {ENVIRONMENT VARIABLE} cargo run -- ["QUERY"] [EXPRESSION] [PATH]
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