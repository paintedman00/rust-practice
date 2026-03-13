use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name", about = "A simple encryption/decryption CLI tool")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        #[clap(short, long, value_parser, help = "Cipher type (caesar or xor)")]
        cipher_type: String,
        #[clap(short, long, value_parser, help = "Key (integer)")]
        key: i32,
        #[clap(short, long, value_parser, help = "Input string")]
        input: String,
    },
    Decrypt {
        #[clap(short, long, value_parser, help = "Cipher type (caesar or xor)")]
        cipher_type: String,
        #[clap(short, long, value_parser, help = "Key (integer)")]
        key: i32,
        #[clap(short, long, value_parser, help = "Input string")]
        input: String,
    },
}

fn caesar_cipher(text: &str, key: i32, encrypt: bool) -> String {
    let shift = if encrypt { key } else { -key };
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let start = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted_char = (((c as u8 - start) as i32 + shift).rem_euclid(26) as u8 + start) as char;
                shifted_char
            } else {
                c
            }
        })
        .collect()
}

fn xor_cipher(text: &str, key: i32) -> String {
    text.chars()
        .map(|c| ((c as u32) ^ (key as u32) ) as u8 as char)
        .collect()
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encrypt { cipher_type, key, input } => {
            let result = match cipher_type.as_str() {
                "caesar" => caesar_cipher(input, *key, true),
                "xor" => xor_cipher(input, *key),
                _ => "Invalid cipher type".to_string(),
            };
            println!("{}", result);
        }
        Commands::Decrypt { cipher_type, key, input } => {
            let result = match cipher_type.as_str() {
                "caesar" => caesar_cipher(input, *key, false),
                "xor" => xor_cipher(input, *key),
                _ => "Invalid cipher type".to_string(),
            };
            println!("{}", result);
        }
    }
}
