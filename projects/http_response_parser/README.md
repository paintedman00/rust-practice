# http_response_parser

A simple command-line tool to parse basic HTTP responses.

## Usage

```bash
http_response_parser "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Hello, world!\"}"
```

This will output a JSON representation of the parsed response, including status line and headers.

## Example Output

```json
{
  "status_line": "HTTP/1.1 200 OK",
  "headers": {
    "Content-Type": "application/json"
  },
  "body": "{\"message\": \"Hello, world!\"}"
}
```
