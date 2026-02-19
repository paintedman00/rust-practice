# Directory Tree Display

A simple command-line tool to visualize a directory tree structure.

## Usage

```bash
cargo run [directory]
```

Replace `[directory]` with the path to the directory you want to visualize. If no directory is provided, the current directory is used.

## Example

```
$ cargo run .
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    └── debug
        ├── build
        ├── deps
        │   └── dir_tree_display-93f5c82604618ef9.d
        ├── examples
        ├── dir_tree_display
        ├── dir_tree_display.d
        └── incremental
            └── dir_tree_display-350x57l34w95n
                └── s-4h3z5pijqv-148r15w-dep-graph.lock
```
