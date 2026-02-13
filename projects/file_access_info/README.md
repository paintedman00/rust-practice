# File Access Info

A simple command-line tool written in Rust to display file access information (permissions) in a human-readable format.

## Usage

```bash
file_access_info <file_path>
```

## Example

```bash
file_access_info /path/to/my/file.txt
```

## Output

The program will output information such as file type, permissions (read, write, execute for owner, group, and others).

## Building

```bash
cargo build --release
```

## Installing

```bash
cargo install --path .
```
