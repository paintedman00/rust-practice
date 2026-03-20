use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut name: Option<String> = None;
    let mut age: Option<i32> = None;
    let mut verbose = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--name" => {
                if i + 1 < args.len() {
                    name = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --name requires an argument");
                    print_help();
                    return;
                }
            }
            "--age" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<i32>() {
                        Ok(num) => age = Some(num),
                        Err(_) => {
                            eprintln!("Error: --age requires an integer argument");
                            print_help();
                            return;
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("Error: --age requires an argument");
                    print_help();
                    return;
                }
            }
            "--verbose" => {
                verbose = true;
                i += 1;
            }
            "--help" => {
                print_help();
                return;
            }
            _ => {
                eprintln!("Error: Unknown argument {}", args[i]);
                print_help();
                return;
            }
        }
    }

    if verbose {
        println!("Arguments parsed:");
    }

    if let Some(name) = name {
        if verbose {
            println!("  Name: {}", name);
        }
        println!("Hello, {}!".to_string(), name);
    }

    if let Some(age) = age {
        if verbose {
            println!("  Age: {}", age);
        }
         println!("You are {} years old.".to_string(), age);
    }

    if name.is_none() && age.is_none() {
        println!("No arguments provided.");
    }
}

fn print_help() {
    println!("Usage: cli_arg_parser --name <name> --age <age> --verbose --help");
}
