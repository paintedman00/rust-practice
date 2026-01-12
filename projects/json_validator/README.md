# json_validator

A simple command-line tool to validate the structure of JSON files against a predefined schema.

## Usage

```bash
json_validator <schema_file> <json_file>
```

*   `<schema_file>`: Path to the JSON schema file.
*   `<json_file>`: Path to the JSON file to validate.

## Example

Schema (schema.json):

```json
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "age": {
      "type": "integer"
    }
  },
  "required": [
    "name",
    "age"
  ]
}
```

JSON to validate (data.json):

```json
{
  "name": "John Doe",
  "age": 30
}
```

Run the validator:

```bash
json_validator schema.json data.json
```

The program will output 'Valid JSON' if the JSON file adheres to the schema or 'Invalid JSON' with a specific error message if it doesn't.
