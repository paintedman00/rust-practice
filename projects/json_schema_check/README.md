# json_schema_check

A simple command-line tool to validate JSON structures against a basic schema.

## Usage

```bash
cargo run -- <path_to_json_file> <schema_definition>
```

`<path_to_json_file>`: Path to the JSON file you want to validate.
`<schema_definition>`: A string defining the expected keys in the JSON object (comma separated).

## Example

To check if `data.json` has the keys `name,age`:

```bash
cargo run -- data.json name,age
```

## Functionality

This tool reads a JSON file, parses it, and validates that the top-level JSON object contains the keys specified in the schema definition.
It provides simple success/failure messages.

## Limitations

- Only validates the presence of keys in the root object.
- No type checking is performed.
- Does not support nested schemas or arrays.
