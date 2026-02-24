use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: text_navigator <pattern> <file>");
        process::exit(1);
    }

    let pattern = &args[1];
    let filename = &args[2];

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    for line in contents.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}
