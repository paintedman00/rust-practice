# env_peek

A simple command-line tool to inspect environment variables and their values.

## Usage

Run the program with `cargo run`. It will print a JSON object to stdout containing all environment variables and their values.

```bash
cargo run
```

## Example Output

```json
{
  "PATH": "/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin",
  "HOME": "/home/user",
  "USER": "user"
  // ... other environment variables
}
```
