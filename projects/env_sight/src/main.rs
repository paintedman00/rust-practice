use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let prefix = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    match prefix {
        Some(p) => {
            let filtered_vars: Vec<(String, String)> = env::vars()
                .filter(|(key, _)| key.starts_with(&p))
                .collect();

            if filtered_vars.is_empty() {
                eprintln!("No environment variables found with prefix: {}", p);
                process::exit(1);
            }

            for (key, value) in filtered_vars {
                println!("{}: {}", key, value);
            }
        }
        None => {
            let all_vars: Vec<(String, String)> = env::vars().collect();
            if all_vars.is_empty() {
                println!("No environment variables found.");
            }

            for (key, value) in all_vars {
                println!("{}: {}", key, value);
            }
        }
    };
}
