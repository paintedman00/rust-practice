# Text Cipher CLI

A simple command-line tool for encrypting and decrypting text using Caesar and XOR ciphers.

## Usage

```bash
text_cipher --cipher <caesar|xor> --action <encrypt|decrypt> --key <integer|string> --input <text>
```

## Arguments

*   `--cipher`: The cipher to use (caesar or xor).
*   `--action`: Whether to encrypt or decrypt.
*   `--key`: The encryption/decryption key. For Caesar, this is an integer shift. For XOR, this is a string.
*   `--input`: The text to encrypt or decrypt.

## Examples

Encrypting with Caesar cipher:

```bash
text_cipher --cipher caesar --action encrypt --key 3 --input "hello"
```

Decrypting with Caesar cipher:

```bash
text_cipher --cipher caesar --action decrypt --key 3 --input "khoor"
```

Encrypting with XOR cipher:

```bash
text_cipher --cipher xor --action encrypt --key "secret" --input "hello"
```

Decrypting with XOR cipher:

```bash
text_cipher --cipher xor --action decrypt --key "secret" --input "Y\S"
```
