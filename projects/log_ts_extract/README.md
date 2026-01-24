# Log Timestamp Extractor

A simple command-line tool to extract timestamps from log files.

## Usage

```bash
log_ts_extract <log_file_path>
```

The program will print the timestamps found in the specified log file to standard output, one timestamp per line.

## Example

If your log file contains lines like:

```
2023-10-27 10:00:00 Some log message
2023-10-27 10:01:00 Another log message
```

Running `log_ts_extract my_log.txt` will output:

```
2023-10-27 10:00:00
2023-10-27 10:01:00
```

## Installation

Make sure you have Rust installed. Clone this repository and run:

```bash
cargo build --release
```

The executable will be located in `target/release/log_ts_extract`.
