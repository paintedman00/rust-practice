# Directory Tree Visualizer

A simple command-line tool to visualize a directory tree as a JSON structure.

## Usage

```bash
cargo run -- <directory_path>
```

Replace `<directory_path>` with the path to the directory you want to visualize. If no path is provided, the current directory is used.

## Example Output

```json
{
  "name": ".",
  "children": [
    {
      "name": "file1.txt",
      "type": "file"
    },
    {
      "name": "subdir",
      "type": "directory",
      "children": [
        {
          "name": "file2.txt",
          "type": "file"
        }
      ]
    }
  ],
  "type": "directory"
}
```
