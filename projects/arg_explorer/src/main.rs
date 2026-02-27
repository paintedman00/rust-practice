use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No arguments provided.");
        return;
    }

    println!("Program name: {}", args[0]);

    let mut name = "World".to_string();
    let mut number = 1;
    let mut verbose = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" => {
                println!("Usage: arg_explorer [options] <arguments>");
                println!("Options:");
                println!("  --name <name>  Sets the name to greet.");
                println!("  -n <number>  Sets the number of times to greet.");
                println!("  -v             Enables verbose output.");
                println!("  --help         Displays this help message.");
                return;
            }
            "--name" => {
                i += 1;
                if i < args.len() {
                    name = args[i].clone();
                } else {
                    eprintln!("Error: Missing value for --name");
                    return;
                }
            }
            "-n" => {
                i += 1;
                if i < args.len() {
                    number = args[i].parse().unwrap_or(1);
                } else {
                    eprintln!("Error: Missing value for -n");
                    return;
                }
            }
            "-v" => {
                verbose = true;
            }
            _ => {
                if verbose {
                    println!("Processing argument: {}", args[i]);
                }
            }
        }
        i += 1;
    }

    for _ in 0..number {
        println!("Hello, {}!", name);
    }
}
