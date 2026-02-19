use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir().unwrap()
    };

    display_tree(&path, "");
}

fn display_tree(path: &PathBuf, prefix: &str) {
    println!("{}{}", prefix, path.file_name().unwrap().to_string_lossy());

    if path.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(path)
            .unwrap()
            .map(|res| res.unwrap().path())
            .collect();
        entries.sort();

        let count = entries.len();
        for (i, entry) in entries.iter().enumerate() {
            let is_last = i == count - 1;
            let new_prefix = format!("{}{}", prefix, if is_last { "└── " } else { "├── " });
            let deeper_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            display_tree(entry, &deeper_prefix);
        }
    }
}
