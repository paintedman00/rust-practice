use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let path = PathBuf::from(file_path);

    let metadata = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("Error getting metadata for {}: {}", file_path, e);
            std::process::exit(1);
        }
    };

    let permissions = metadata.permissions();
    let mode = permissions.mode();

    let file_type = if metadata.is_dir() {
        "Directory"
    } else if metadata.is_file() {
        "File"
    } else {
        "Other"
    };

    println!("File Type: {}", file_type);
    println!("Permissions (octal): {:o}", mode & 0o777);
    println!("Owner: Read={}, Write={}, Execute={}", (mode & 0o400) != 0, (mode & 0o200) != 0, (mode & 0o100) != 0);
    println!("Group: Read={}, Write={}, Execute={}", (mode & 0o040) != 0, (mode & 0o020) != 0, (mode & 0o010) != 0);
    println!("Other: Read={}, Write={}, Execute={}", (mode & 0o004) != 0, (mode & 0o002) != 0, (mode & 0o001) != 0);
}
