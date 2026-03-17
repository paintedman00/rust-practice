use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use clap::Parser;

/// A simple program to display the hexadecimal representation of a file.
#[derive(Parser, Debug)]
#[clap(author = "Your Name", version, about = "A simple hex dump utility", long_about = None)]
struct Args {
    /// Path to the file to be hex dumped
    #[clap(name = "FILE", parse(from_os_str), value_hint = clap::ValueHint::FilePath)]
    file_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut file = File::open(args.file_path)?; 
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    for (i, byte) in buffer.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
    println!();

    Ok(())
}
