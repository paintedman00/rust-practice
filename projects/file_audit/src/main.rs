use std::fs;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: file_audit <file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let path = Path::new(file_path);

    if !path.exists() {
        eprintln!("Error: File '{}' does not exist.", file_path);
        std::process::exit(1);
    }

    match fs::metadata(path) {
        Ok(metadata) => {
            let permissions = metadata.permissions();
            let mode = permissions.mode();

            println!("File: {}", file_path);
            println!("Permissions (octal): {:o}", mode);

            // Basic permission breakdown
            let owner_read = (mode & 0o400) != 0;
            let owner_write = (mode & 0o200) != 0;
            let owner_execute = (mode & 0o100) != 0;

            let group_read = (mode & 0o040) != 0;
            let group_write = (mode & 0o020) != 0;
            let group_execute = (mode & 0o010) != 0;

            let others_read = (mode & 0o004) != 0;
            let others_write = (mode & 0o002) != 0;
            let others_execute = (mode & 0o001) != 0;

            println!("Owner: read={}, write={}, execute={}", owner_read, owner_write, owner_execute);
            println!("Group: read={}, write={}, execute={}", group_read, group_write, group_execute);
            println!("Others: read={}, write={}, execute={}", others_read, others_write, others_execute);
        }
        Err(e) => {
            eprintln!("Error: Could not read metadata for '{}': {}", file_path, e);
            std::process::exit(1);
        }
    }
}
