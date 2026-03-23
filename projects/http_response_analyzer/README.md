# http_response_analyzer

A simple command-line tool to analyze basic HTTP responses.

## Usage

Provide an HTTP response string as input, and the program will attempt to parse and display some key information.

```bash
cargo run "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Hello, world!\"}"
```

## Example Output

```json
{
  "status_code": 200,
  "content_type": "application/json"
}
```
