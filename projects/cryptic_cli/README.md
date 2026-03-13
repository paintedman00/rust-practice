# Cryptic CLI

A simple command-line tool for basic encryption and decryption using Caesar or XOR ciphers.

## Usage

```bash
cryptic_cli encrypt -t caesar -k 3 -i "Hello, world!"
cryptic_cli decrypt -t caesar -k 3 -i "Khoor, zruog!"

cryptic_cli encrypt -t xor -k 42 -i "Secret message"
cryptic_cli decrypt -t xor -k 42 -i "\u001a\u0010\u000b\u0012\u001b G\u0004\u0019\u0006\u0013\u0010\u0004\u001b"
```

## Arguments

*   `encrypt`: Encrypt the input.
*   `decrypt`: Decrypt the input.
*   `-t`: Cipher type (`caesar` or `xor`).
*   `-k`: Key (integer).
*   `-i`: Input string.
