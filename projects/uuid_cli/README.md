# uuid_cli

A simple command-line UUID generator written in Rust.

## Usage

Run the program with `cargo run` to generate a single UUID v4.

```bash
cargo run
```

To generate a specific number of UUIDs, provide the count as an argument:

```bash
cargo run -- 5
```

This will output 5 UUIDs.

## Building

Make sure you have Rust installed. Then, clone the repository and build the project:

```bash
git clone <repository_url>
cd uuid_cli
cargo build --release
```

The executable will be located in `target/release/uuid_cli`.

## License

MIT
