use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: greeter <name>");
        std::process::exit(1);
    }

    let name = &args[1];
    println!("Hello, {}!", name);
}
