use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filter = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    let mut found = false;

    for (key, value) in env::vars() {
        match &filter {
            Some(f) => {
                if key.contains(f) || value.contains(f) {
                    println!("{}: {}", key, value);
                    found = true;
                }
            }
            None => {
                println!("{}: {}", key, value);
                found = true;
            }
        }
    }

    if !found {
        match &filter {
            Some(f) => {
                eprintln!("No environment variables found matching filter: {}", f);
                process::exit(1);
            }
            None => {
                eprintln!("No environment variables found.");
                process::exit(1);
            }
        }
    }
}
