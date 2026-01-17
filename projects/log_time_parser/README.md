# Log Timestamp Parser

A simple command-line tool written in Rust to parse timestamps from log files.

## Usage

```bash
log_time_parser <log_file> <timestamp_format>
```

*   `<log_file>`: Path to the log file.
*   `<timestamp_format>`:  A string representing the format of the timestamp in the log file.  For example, `%Y-%m-%d %H:%M:%S`. Uses chrono format specifiers. See: https://docs.rs/chrono/latest/chrono/format/strftime/index.html

## Example

If your log file `example.log` contains lines like:

```
2023-10-27 10:00:00 Some log message
2023-10-27 10:01:00 Another log message
```

You can use the tool like this:

```bash
log_time_parser example.log "%Y-%m-%d %H:%M:%S"
```

This will output the timestamps from the log file.
