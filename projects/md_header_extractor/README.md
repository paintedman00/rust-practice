# Markdown Header Extractor

A simple command-line tool to extract headers from a Markdown file.

## Usage

```bash
md_header_extractor <input_file>
```

For example:

```bash
md_header_extractor example.md
```

## Example

If `example.md` contains:

```markdown
# Title

## Subtitle

Some text.

### Section
```

the output will be:

```
Title
Subtitle
Section
```
