# Timestamp Inspector

A simple command-line tool to parse timestamps from log files and output them in a structured format.

## Usage

```bash
cargo run -- <log_file_path>
```

For example:

```bash
cargo run -- ./sample.log
```

This will read the log file `sample.log`, attempt to extract timestamps from each line, and print them in JSON format (if any are found).

## Example `sample.log`

```
2023-10-27 10:00:00 Some log message here.
Another line without timestamp.
2023-10-27 10:01:00 Another log message.
```

## Output

The tool will output a JSON array containing the extracted timestamps.  If no timestamps are found, an empty array will be output.

Example output for the `sample.log` shown above:

```json
[
  "2023-10-27 10:00:00",
  "2023-10-27 10:01:00"
]
```
