# csv_jsonify

A simple command-line tool to convert CSV files to JSON format.

## Usage

```bash
csv_jsonify input.csv output.json
```

## Example

If `input.csv` contains:

```csv
name,age,city
John,30,New York
Jane,25,London
```

Then `output.json` will contain:

```json
[{"name":"John","age":"30","city":"New York"},{"name":"Jane","age":"25","city":"London"}]
```
