use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <pattern> <file_path>", args[0]);
        process::exit(1);
    }

    let pattern = &args[1];
    let file_path = &args[2];

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file `{}`: {}", file_path, err);
            process::exit(1);
        }
    };

    for line in contents.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}
