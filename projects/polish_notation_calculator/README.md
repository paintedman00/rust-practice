# Polish Notation Calculator

A simple command-line Reverse Polish Notation (RPN) calculator written in Rust.

## Usage

Run the program with `cargo run`.

Supported operations: `+`, `-`, `*`, `/`.

Enter numbers and operators separated by spaces. The calculator will evaluate the expression and print the result.

Example:

```
> 5 2 +
7
> 10 5 / 2 *
4
```

## Error Handling

The calculator handles the following errors:

*   Invalid input (non-numeric or unsupported operator)
*   Not enough operands for an operation
*   Division by zero

## Future Improvements

*   Add support for more operators (e.g., exponentiation, modulo).
*   Implement error handling for floating point numbers.
*   Improve user interface with a REPL.
