/*
    env: Terminal;
    error: Std error trait;
    fs: Filesystem manipulation operations.
*/
use std::{env, error::Error, fs};

// Program command structure
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // Build the program configuration based on command arguments.
    // Arguments should include the query, file path, and optionally, the option to ignore case.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Args need to have at least query and file_path to run
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        // Toggle the case-sensitive of the command.
        // arg takes the precendence over the environment
        // variable if they are opposite to each other.
        let ignore_case = args.get(3).map_or_else(
            || env::var("IGNORE_CASE").is_ok(),
            |arg| match arg.as_str() {
                "-i" | "--ignore-case" => true,
                "-ni" | "--no-ignore-case" => false,
                _ => false,
            },
        );

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// That's the core function of the program.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // Choose the case sensitive according to the value of ignore_case field
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// No case sensitive string query search.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// Case sensitive string query search.
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests the search for words in a case-sensitive manner.
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // Tests the search for words in a case-insensitive manner.
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    // Tests if the option to ignore case is applied when defined twice,
    // once by the environment and once by the argument.
    #[test]
    fn double_opposite_case() {
        // Sets the environment variable to ignore case.
        std::env::set_var("IGNORE_CASE", "1");

        // Sets the command arguments, including the option to ignore case.
        let args = vec![
            "".to_string(),
            "to".to_string(),
            "poem.txt".to_string(),
            "-i".to_string(),
        ];
        // Creates the configuration based on the arguments.
        let config = Config::build(&args).unwrap();

        // Checks if the configuration indicates that comparison should be case-insensitive.
        assert_eq!(true, config.ignore_case);
    }
}
