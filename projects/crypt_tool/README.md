# Crypt Tool

A simple command-line tool for basic encryption and decryption using Caesar and XOR ciphers.

## Usage

```bash
crypt_tool --algorithm caesar --action encrypt --key 3 --input "Hello, World!"
crypt_tool --algorithm xor --action decrypt --key 42 --input "Some encrypted text"
```

## Arguments

*   `--algorithm`: The encryption algorithm to use (caesar or xor). Defaults to caesar.
*   `--action`: The action to perform (encrypt or decrypt). Defaults to encrypt.
*   `--key`: The encryption key (integer). Required.
*   `--input`: The text to encrypt or decrypt. Required.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
