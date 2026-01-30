use std::env;
use rand::Rng;

fn generate_uuid() -> String {
    let mut rng = rand::thread_rng();
    let uuid = rng.gen::<u128>();
    format!("{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        (uuid >> 96) as u32,
        ((uuid >> 80) & 0xffff) as u16,
        ((uuid >> 64) & 0xffff) as u16,
        ((uuid >> 48) & 0xffff) as u16,
        (uuid & 0xffffffffffff) as u64
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut count = 1;

    if args.len() > 1 {
        if args[1] == "--count" {
            if args.len() > 2 {
                if let Ok(n) = args[2].parse::<i32>() {
                    count = n;
                } else {
                    eprintln!("Invalid number provided for --count");
                    std::process::exit(1);
                }
            } else {
                eprintln!("Missing argument for --count");
                std::process::exit(1);
            }
        }
    }

    for _ in 0..count {
        println!("{}", generate_uuid());
    }
}
