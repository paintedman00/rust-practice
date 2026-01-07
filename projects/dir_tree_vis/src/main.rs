use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct FileSystemNode {
    name: String,
    node_type: String,
    children: Option<Vec<FileSystemNode>>,
}

fn build_tree(path: &Path) -> FileSystemNode {
    let name = path.file_name().unwrap().to_string_lossy().to_string();
    if path.is_file() {
        FileSystemNode {
            name,
            node_type: "file".to_string(),
            children: None,
        }
    } else {
        let mut children = Vec::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    children.push(build_tree(&entry.path()));
                }
            }
        }
        FileSystemNode {
            name,
            node_type: "directory".to_string(),
            children: Some(children),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    let tree = build_tree(path);

    let json_string = serde_json::to_string_pretty(&tree).unwrap();
    println!("{}", json_string);
}
