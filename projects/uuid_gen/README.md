# uuid_gen

A simple command-line UUID generator written in Rust.

## Usage

Run the program from your terminal:

```bash
cargo run
```

This will generate a single UUID v4 and print it to the console.

Options:

*   `-n <number>` or `--number <number>`: Generate a specified number of UUIDs. Defaults to 1.

```bash
cargo run -- -n 5
```

This will generate 5 UUIDs and print them each on a new line.

## Building

```bash
cargo build
```

## Installation

```bash
cargo install --path .
```

## License

MIT
