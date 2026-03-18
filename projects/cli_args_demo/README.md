# cli_args_demo

A simple command-line argument parsing demo in Rust.

## Usage

```bash
cargo run -- [options]
```

Options:

*   `-n, --name <name>`: Sets a custom name (string).
*   `-a, --age <age>`: Sets an age (integer).
*   `-v, --verbose`: Enables verbose output (boolean flag).
*   `-h, --help`: Shows help information.

## Examples

```bash
cargo run -- -n Alice -a 30 -v
cargo run -- --name Bob --age 25
cargo run -- -h
```
