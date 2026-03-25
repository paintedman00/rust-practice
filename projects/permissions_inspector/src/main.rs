use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: permissions_inspector <file_path>");
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

    let user_read = (mode & 0o400) != 0;
    let user_write = (mode & 0o200) != 0;
    let user_execute = (mode & 0o100) != 0;

    let group_read = (mode & 0o040) != 0;
    let group_write = (mode & 0o020) != 0;
    let group_execute = (mode & 0o010) != 0;

    let others_read = (mode & 0o004) != 0;
    let others_write = (mode & 0o002) != 0;
    let others_execute = (mode & 0o001) != 0;

    println!("File: {}", file_path);
    println!("User:   read={}, write={}, execute={}", user_read, user_write, user_execute);
    println!("Group:  read={}, write={}, execute={}", group_read, group_write, group_execute);
    println!("Others: read={}, write={}, execute={}", others_read, others_write, others_execute);
}
