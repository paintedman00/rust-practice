use std::fs;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: file_perm_analyzer <file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let path = PathBuf::from(file_path);

    if !path.exists() {
        eprintln!("Error: File not found: {}", file_path);
        std::process::exit(1);
    }

    match fs::metadata(&path) {
        Ok(metadata) => {
            let permissions = metadata.permissions();
            let mode = permissions.mode();

            println!("File: {}", file_path);
            println!("Permissions (octal): {:o}", mode);

            // Owner permissions
            let owner_read = (mode & 0o400) != 0;
            let owner_write = (mode & 0o200) != 0;
            let owner_execute = (mode & 0o100) != 0;

            println!("Owner: Read={}, Write={}, Execute={}", owner_read, owner_write, owner_execute);

            // Group permissions
            let group_read = (mode & 0o040) != 0;
            let group_write = (mode & 0o020) != 0;
            let group_execute = (mode & 0o010) != 0;

            println!("Group: Read={}, Write={}, Execute={}", group_read, group_write, group_execute);

            // Others permissions
            let others_read = (mode & 0o004) != 0;
            let others_write = (mode & 0o002) != 0;
            let others_execute = (mode & 0o001) != 0;

            println!("Others: Read={}, Write={}, Execute={}", others_read, others_write, others_execute);
        }
        Err(e) => {
            eprintln!("Error reading metadata for {}: {}", file_path, e);
            std::process::exit(1);
        }
    }
}
