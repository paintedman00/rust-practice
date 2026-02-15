# Reverse Polish Notation Calculator

A simple command-line Reverse Polish Notation (RPN) calculator written in Rust.

## Usage

```bash
cargo run <expression>
```

Where `<expression>` is a space-separated RPN expression.

## Example

```bash
cargo run 5 2 +
```

This will output:

```
7
```

## Supported Operators

*   `+` (Addition)
*   `-` (Subtraction)
*   `*` (Multiplication)
*   `/` (Division)

## Error Handling

The program provides basic error handling for invalid input and division by zero.
