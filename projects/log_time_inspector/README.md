# Log Time Inspector

A simple command-line tool to parse and extract timestamps from log files.

## Usage

```bash
log_time_inspector <log_file_path>
```

This will output the timestamps found in the specified log file.

## Example

If your log file `example.log` contains the following:

```
2023-10-27 10:00:00 Some log message
2023-10-27 10:01:00 Another log message
```

Running `log_time_inspector example.log` might output (the exact format depends on the internal parsing):

```
2023-10-27 10:00:00
2023-10-27 10:01:00
```
