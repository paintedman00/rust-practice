# crypt_cli

A simple command-line tool for basic encryption (Caesar and XOR).

## Usage

```bash
crypt_cli <mode> <key> <input_string>
```

*   `<mode>`: `caesar` or `xor`
*   `<key>`: Integer for Caesar cipher shift or hexadecimal key for XOR cipher.
*   `<input_string>`: The string to encrypt or decrypt.

## Examples

```bash
# Caesar encryption with a shift of 3
crypt_cli caesar 3 "Hello, world!"

# XOR encryption with key 42
crypt_cli xor 2a "Hello, world!"
```
