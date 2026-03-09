# File Size Histogram

This is a simple command-line tool that generates a histogram of file sizes in a given directory.

## Usage

```bash
file_size_histo <directory>
```

For example:

```bash
file_size_histo ./data
```

This will print a histogram to the console, showing the distribution of file sizes in the `data` directory.

## Example Output

```
0-1KB:   ******
1-10KB:  *********
10-100KB: ****
100KB-1MB: *
>1MB:      **
```
