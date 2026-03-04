use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    print_tree(Path::new(path), "");
}

fn print_tree(path: &Path, prefix: &str) {
    println!("{}{}", prefix, path.file_name().unwrap_or_default().to_string_lossy());

    if path.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(path)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .filter_map(|res| res.ok())
            .collect();

        entries.sort();

        let mut iter = entries.iter().peekable();

        while let Some(entry) = iter.next() {
            let is_last = iter.peek().is_none(); // Check if it's the last entry

            let new_prefix = if is_last {
                format!("{}└── ", prefix)
            } else {
                format!("{}├── ", prefix)
            };

            let deeper_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            print_tree(entry, &deeper_prefix);
        }
    }
}
