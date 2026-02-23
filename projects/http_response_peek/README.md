# http_response_peek

A simple command-line tool to parse and display parts of an HTTP response.

## Usage

```bash
http_response_peek <http_response_string>
```

For example:

```bash
http_response_peek "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Hello, world!\"}"
```

This will output a JSON representation of the parsed response, including the status line, headers, and body.
