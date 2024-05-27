# My Rust Program

```markdown
This program is a command-line utility written in Rust that searches for a query in given input (either a file or direct text input).
```

## Usage in Linux Terminal

### File Path
```bash
cargo run -- "QUERY" "PATH" [-i|--ignore-case | -ni|--no-ignore-case]
```

### String Literal One Line
```bash
echo "The literal string go here" | cargo run -- "QUERY" [-i|--ignore-case | -ni|--no-ignore-case]
```

### String Literal Multiple Lines
```bash
cat << EOF | cargo run -- "QUERY" [-i|--ignore-case | -ni|--no-ignore-case]
The literal string
go
here
EOF
```

## Options
- `-i, --ignore-case`: Ignore case sensitivity in the search.
- `-ni, --no-ignore-case`: Do not ignore case sensitivity in the search.
- `-h, --help`: Display the help message and exit.

## Environment Variables
- `IGNORE_CASE=1`: Ignore case sensitivity in the search.

## Examples
Search in a file with case-insensitive mode
```bash
cargo run -- "rust" "src/main.rs" -i
```

Search with string literal one line with case-sensitive mode
```bash
echo "The quick brown fox jumps over the lazy dog" | cargo run -- "fox"
```

Search with string literal multiple lines with case-insensitive mode
``` bash
cat << EOF | cargo run -- "lazy" -i
The quick brown fox
jumps over the lazy dog.
EOF
```
Running Tests
```bash
cargo test
```