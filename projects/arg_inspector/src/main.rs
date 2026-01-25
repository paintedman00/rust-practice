use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Program Name: {}", args[0]);

    if args.len() == 1 {
        println!("No arguments provided.");
        return;
    }

    println!("Arguments provided:");
    for (i, arg) in args.iter().enumerate() {
        if i == 0 { continue; } // Skip the program name
        println!("  Argument {}: {}", i, arg);
    }

    // Example usage of checking for specific flags
    if args.contains(&String::from("--help")) {
        println!("\nUsage: arg_inspector [arguments]");
        println!("  --help: Display this help message");
        // Add more options as needed
    }


    // Example of handling a specific argument
    if let Some(index) = args.iter().position(|x| x == "--name") {
        if index + 1 < args.len() {
            println!("\nName provided: {}", &args[index + 1]);
        } else {
            println!("\nError: --name flag requires a value.");
        }
    }
}
