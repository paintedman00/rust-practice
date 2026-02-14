# csv_to_json

A simple command-line tool to convert CSV files to JSON format.

## Usage

```bash
cargo run input.csv output.json
```

Replace `input.csv` with the path to your CSV file and `output.json` with the desired output file name.

## Example

If `input.csv` contains:

```csv
name,age,city
Alice,30,New York
Bob,25,London
```

Then `output.json` will contain:

```json
[{"name":"Alice","age":"30","city":"New York"},{"name":"Bob","age":"25","city":"London"}]
```
