# Simple Cipher CLI

A simple command-line tool for basic encryption and decryption using Caesar cipher and XOR cipher.

## Usage

```bash
# Caesar cipher encryption
simple_cipher_cli encrypt caesar -k <key> -i <input_string>

# Caesar cipher decryption
simple_cipher_cli decrypt caesar -k <key> -i <input_string>

# XOR cipher encryption
simple_cipher_cli encrypt xor -k <key> -i <input_string>

# XOR cipher decryption
simple_cipher_cli decrypt xor -k <key> -i <input_string>
```

## Arguments

*   `-k`, `--key` : The encryption/decryption key (integer for Caesar, string for XOR).
*   `-i`, `--input` : The input string to encrypt or decrypt.

## Examples

```bash
simple_cipher_cli encrypt caesar -k 3 -i "hello"
# Output: khoor

simple_cipher_cli decrypt caesar -k 3 -i "khoor"
# Output: hello

simple_cipher_cli encrypt xor -k mykey -i "hello"
# Output: Example XOR output.
```

## Contributing

Feel free to contribute to this project by submitting pull requests.
