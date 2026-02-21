# Text Analyzer

A simple command-line tool to analyze text files, providing word, line, and byte counts.

## Usage

```bash
text_analyzer <file_path>
```

## Example

```bash
text_analyzer example.txt
```

This will output the number of lines, words, and bytes in `example.txt`.

If no file path is provided, it reads from stdin.

## Building

```bash
cargo build --release
```

## Installing

```bash
cargo install --path .
```
