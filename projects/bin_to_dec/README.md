# Binary to Decimal Converter

A simple command-line tool written in Rust that converts binary numbers to their decimal equivalent.

## Usage

```bash
bin_to_dec <binary_number>
```

For example:

```bash
bin_to_dec 101010
```

Will output:

```
42
```

## Building

Make sure you have Rust installed. Then, clone this repository and run:

```bash
cargo build --release
```

## Installing

After building, you can install the binary to your system's PATH:

```bash
cargo install --path .
```
