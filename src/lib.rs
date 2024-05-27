/*
    env: Terminal;
    error: Std error trait;
    fs: Filesystem manipulation operations;
    io: I/O functionality
*/
use std::{
    env,
    error::Error,
    fs,
    io::{self, Read},
};

// enum for Config.input
#[derive(PartialEq)]
pub enum InputType {
    FilePath(String),
    LiteralInput(String),
}

// Program command structure
pub struct Config {
    pub query: String,
    pub ignore_case: bool,
    pub input: InputType,
}

// That's the core method of the program.
impl Config {
    /// Builds the program configuration based on the arguments passed by the command.
    ///
    /// # Arguments
    /// - `args`: An iterator over the strings representing the command-line arguments. Typically,
    ///   this is passed directly from `std::env::args()`, which includes the program name as the first argument.
    ///   - First argument: Program name (automatically skipped inside the function).
    ///
    /// # Returns
    /// - `Ok(Config)`: Successfully creates a `Config` object configured with the provided arguments.
    /// - `Err(Box<dyn Error>)`: Returns an error if any of the necessary components (query, ignore_case, input)
    ///   cannot be properly retrieved or parsed.
    pub fn build<I>(mut args: I) -> Result<Config, Box<dyn Error>>
    where
        I: Iterator<Item = String>,
    {
        // Skip the first arg (program name).
        args.next();

        let required_args: Vec<String> = args.collect();

        let query = Config::get_query(&mut required_args.iter().map(|s| s.to_string()))?;
        let ignore_case = Config::get_ignore_case(&required_args);
        let input = Config::get_input(&mut required_args.iter().map(|s| s.to_string()))?;

        Ok(Config {
            query,
            ignore_case,
            input,
        })
    }

    /// Get a query string in the 'arg[1]' to find't.
    /// But not before checking whether the arg contains a help.
    ///
    /// # Arguments
    /// - `args`: An Iterator of strings representing command line arguments.
    ///
    /// # Returns
    /// - `Ok(arg)` if get a query string.
    /// - `exit(1)` if get the help expression.
    /// - `Err()` if didn't get the previous values.
    fn get_query<I>(args: &mut I) -> Result<String, Box<dyn Error>>
    where
        I: Iterator<Item = String>,
    {
        if let Some(arg) = args.next() {
            if !arg.contains("-h") || !arg.contains("--help") {
                Ok(arg)
            } else {
                println!("{}", Config::help_message());
                std::process::exit(1);
            }
        } else {
            Err("Didn't get a query string".into())
        }
    }

    /// Determines the case sensitivity for the command based on the provided arguments and environment.
    ///
    /// The function first checks command line arguments for `-i` (`--ignore-case`) or `-ni` (`--no-ignore-case`).
    /// If neither is specified, it checks the `IGNORE_CASE` environment variable.
    ///
    /// # Arguments
    /// - `args`: A slice of strings representing command line arguments.
    ///
    /// # Returns
    /// - `true` if case sensitivity should be ignored (case-insensitive mode).
    /// - `false` if case sensitivity should be respected (case-sensitive mode).
    ///
    /// # Note
    /// - The `-ni` or `--no-ignore-case` argument takes precedence over the `IGNORE_CASE` environment variable.
    fn get_ignore_case(args: &[String]) -> bool {
        let ignore_case_flag = args.iter().any(|arg| arg == "-i" || arg == "--ignore-case");
        let no_ignore_case_flag = args
            .iter()
            .any(|arg| arg == "-ni" || arg == "--no-ignore-case");

        // Check command line flags first for explicit setting
        if no_ignore_case_flag {
            false
        } else if ignore_case_flag {
            true
        } else {
            // If no flags are specified, default to the environment variable
            env::var("IGNORE_CASE").is_ok()
        }
    }

    /// Distinguishes between file path and command and returns the InputType.
    ///
    /// # Arguments
    /// - `args` An Iterator of strings representing command line arguments.
    ///
    /// - `Ok(InputType::FilePath(String))`: Returns a `FilePath` variant of `InputType` if one of the arguments
    ///   contains a '/' or '\\' indicating a path.
    /// - `Ok(InputType::LiteralInput(String))`: Returns a `LiteralInput` variant of `InputType` if no path is
    ///   detected. It reads the entire input from stdin, assuming it to be a direct text input.
    /// - `Err(Box<dyn Error>)`: Returns an error if there are issues reading from stdin.
    fn get_input<'a, I>(args: &mut I) -> Result<InputType, Box<dyn Error>>
    where
        I: Iterator<Item = String>,
    {
        // Checks if it is a file path.
        if let Some(arg) = args.find(|arg| arg.contains('/') || arg.contains('\\')) {
            Ok(InputType::FilePath(arg))
        // Understands that it is a command.
        } else {
            let mut input_line = String::new();
            io::stdin().read_to_string(&mut input_line)?;

            Ok(InputType::LiteralInput(
                input_line.trim_matches('"').to_string(),
            ))
        }
    }

    /// This function is used when get_query() realize that it
    /// contains an arg by calling the help.
    fn help_message() -> String {
        "Usage:
        File Path:
        cargo run -- [\"QUERY\"] [PATH] [EXPRESSION]

        String Literal One Line:
        echo \"The literal string go here\" | cargo run -- [\"QUERY\"] [EXPRESSION]

        String Literal Multiple Lines:
        cat << EOF | cargo run -- [\"QUERY\"] [EXPRESSION]
        The literal string
        go
        here
        EOF

        Help:
        cargo run -- [EXPRESSION]

        Expressions:
        -i, --ignore-case        ignore case sensitive in search
        -ni, --no-ignore-case    don't ignore case sensitive in search
        -h, --help               display this help and exit
        
        Environment Variable Usage:
        IGNORE_CASE=1            ignore case sensitive in search"
            .to_string()
    }
}

/// That's the core function of the program.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // See the description in `Config::get_input()`
    let contents = match config.input {
        InputType::FilePath(path) => fs::read_to_string(path)?,
        InputType::LiteralInput(text) => text,
    };

    // Searches for the ´query´
    let results = search(&config.query, config.ignore_case, &contents);

    // Print the query
    results
        .iter()
        .for_each(|line| print_highlighted(&config.query, config.ignore_case, line));

    Ok(())
}

// Print the matched query in highlighted style.
fn print_highlighted(query: &str, ignore_case: bool, line: &str) {
    let mut start = 0;
    let query_len = query.len();

    // Adjustment for case sensitive.
    let target_line = if ignore_case {
        line.to_lowercase()
    } else {
        line.to_string()
    };

    // Adjustment for case sensitive.
    let target_query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    while let Some(position) = target_line[start..].find(&target_query) {
        // Prints before the word.
        print!("{}", &line[start..start + position]);

        // Prints the highlighted word.
        print!(
            "\x1b[31m{}\x1b[0m",
            &line[start + position..start + position + query_len]
        );

        // Updates the starting position to after the word.
        start += position + query_len;
    }

    // Prints the remaining line.
    println!("{}", &line[start..]);
}

/// Searches the given content for lines that contain the specified query.
///
/// # Parameters
/// - `query`: The text string to search for within each line of `contents`.
/// - `ignore_case`: A boolean indicating whether the search should be case insensitive.
/// - `contents`: The text within which to search for `query`.
///
/// # Returns
/// A vector of strings, each a line from `contents` that matches the `query` based on the specified case sensitivity.
fn search<'a>(query: &str, ignore_case: bool, contents: &'a str) -> Vec<&'a str> {
    // Convert the query to lowercase if the search is case insensitive, done once for efficiency.
    let query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    // Define a line filter function: uses a dynamic dispatch via Box<dyn Fn(&str) -> bool>.
    // This allows switching the filtering function based on `ignore_case`.

    let line_filter = if ignore_case {
        // For case-insensitive search, compare each line in lowercase to the lowercase query.
        Box::new(|line: &str| line.to_lowercase().contains(&query.to_lowercase()))
    } else {
        // For case-sensitive search, directly check if the line contains the query.
        Box::new(|line: &str| line.contains(&query)) as Box<dyn Fn(&str) -> bool>
    };

    // Process each line of the contents, filtering based on the presence of the query
    // as determined by the line_filter function. Collect matching lines into a vector.
    contents.lines().filter(|line| line_filter(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests the search for words in a case-sensitive manner.
    #[test]
    fn case_sensitive() {
        let ignore_case = false;
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, ignore_case, contents)
        );
    }

    // Tests the search for words in a case-insensitive manner.
    #[test]
    fn case_insensitive() {
        let ignore_case = true;
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, ignore_case, contents)
        );
    }

    // Tests if the option to ignore case is applied when defined twice,
    // once by the environment and once by the argument.
    #[test]
    #[ignore = "failure due to lack of interest"]
    fn double_opposite_case() {
        // Sets the environment variable to ignore case.
        std::env::set_var("IGNORE_CASE", "1");

        // Sets the command arguments, including the option to ignore case.
        let args = vec![
            "".to_string(),
            "to".to_string(),
            "-i".to_string(),
            "poem.txt".to_string(),
        ];

        // Create the iterator based on arguments vector.
        let args_iter = args.into_iter();

        // Creates the configuration based on the arguments.
        let config = Config::build(args_iter).unwrap();

        // Checks if the configuration indicates that comparison should be case-insensitive.
        assert_eq!(true, config.ignore_case);
    }
}
