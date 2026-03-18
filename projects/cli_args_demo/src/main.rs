use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut name = "World".to_string();
    let mut age = 0;
    let mut verbose = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-n" | "--name" => {
                if i + 1 < args.len() {
                    name = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: Missing value for name");
                    return;
                }
            }
            "-a" | "--age" => {
                if i + 1 < args.len() {
                    if let Ok(parsed_age) = args[i + 1].parse::<i32>() {
                        age = parsed_age;
                        i += 2;
                    } else {
                        eprintln!("Error: Invalid age value");
                        return;
                    }
                } else {
                    eprintln!("Error: Missing value for age");
                    return;
                }
            }
            "-v" | "--verbose" => {
                verbose = true;
                i += 1;
            }
            "-h" | "--help" => {
                print_help();
                return;
            }
            _ => {
                eprintln!("Error: Unknown argument: {}", args[i]);
                print_help();
                return;
            }
        }
    }

    if verbose {
        println!("Running in verbose mode");
    }

    println!("Hello, {}!", name);
    if age > 0 {
        println!("You are {} years old.", age);
    }
}

fn print_help() {
    println!("Usage: cli_args_demo [options]");
    println!("Options:");
    println!("  -n, --name <name>   Sets a custom name.");
    println!("  -a, --age <age>     Sets an age.");
    println!("  -v, --verbose     Enables verbose output.");
    println!("  -h, --help        Shows this help message.");
}
