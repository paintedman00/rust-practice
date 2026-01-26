# Rust WC (Word Count)

A simple command-line tool written in Rust to count lines, words, and characters in a file or standard input, similar to the `wc` utility.

## Usage

```bash
rust_wc <file>
# or
cat <file> | rust_wc
```

If no file is provided as an argument, the tool reads from standard input.

## Output

The tool outputs the number of lines, words, and characters, followed by the filename (if applicable).
