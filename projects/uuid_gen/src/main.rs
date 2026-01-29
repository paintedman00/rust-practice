use std::str::FromStr;

use clap::Parser;
use uuid::Uuid;

/// Generate UUIDs from the command line.
#[derive(Parser, Debug)]
#[command(author = "", version = "1.0", about = "Generates UUIDs", long_about = None)]
struct Args {
    /// Number of UUIDs to generate
    #[arg(short, long, default_value_t = 1)]
    number: u32,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.number {
        let uuid = Uuid::new_v4();
        println!("{}", uuid);
    }
}
