# Log Timestamp Analyzer

A simple command-line tool to analyze timestamps within log files and extract the earliest and latest timestamps.

## Usage

```bash
log_ts_analyzer <log_file_path>
```

## Example

Assuming you have a log file named `application.log`:

```bash
log_ts_analyzer application.log
```

The tool will output the earliest and latest timestamps found in the log file.

## Output

Example:

```
Earliest Timestamp: 2023-10-26T10:00:00
Latest Timestamp: 2023-10-26T10:30:00
```
