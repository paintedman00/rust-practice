# md_header_list

A simple command-line tool to extract headers from a Markdown file and print them to the console.

## Usage

```bash
md_header_list <path_to_markdown_file>
```

## Example

If `input.md` contains:

```markdown
# Title

## Subtitle

Some text.

### Smaller Subtitle
```

Running `md_header_list input.md` will output:

```
# Title
## Subtitle
### Smaller Subtitle
```
