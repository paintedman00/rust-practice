use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author = "Your Name", version = "1.0", about = "A simple CLI for basic encryption and decryption.", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        #[clap(arg_required_else_help = true)]
        cipher_type: String,
        #[clap(short, long, value_parser, help = "Encryption key")]
        key: String,
        #[clap(short, long, value_parser, help = "Input string")]
        input: String,
    },
    Decrypt {
        #[clap(arg_required_else_help = true)]
        cipher_type: String,
        #[clap(short, long, value_parser, help = "Decryption key")]
        key: String,
        #[clap(short, long, value_parser, help = "Input string")]
        input: String,
    },
}

fn caesar_cipher(text: &str, key: i32, decrypt: bool) -> String {
    let shift = if decrypt { -key } else { key };
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let start = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted_char = (((c as u8 - start as u8) as i32 + shift).rem_euclid(26) + start as i32) as u8 as char;
                shifted_char
            } else {
                c
            }
        })
        .collect()
}

fn xor_cipher(text: &str, key: &str) -> String {
    text.bytes()
        .zip(key.bytes().cycle())
        .map(|(text_byte, key_byte)| (text_byte ^ key_byte) as char)
        .collect()
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encrypt { cipher_type, key, input } => {
            let result = match cipher_type.as_str() {
                "caesar" => caesar_cipher(input, key.parse::<i32>().unwrap(), false),
                "xor" => xor_cipher(input, key),
                _ => panic!("Invalid cipher type"),
            };
            println!("{}", result);
        }
        Commands::Decrypt { cipher_type, key, input } => {
            let result = match cipher_type.as_str() {
                "caesar" => caesar_cipher(input, key.parse::<i32>().unwrap(), true),
                "xor" => xor_cipher(input, key),
                _ => panic!("Invalid cipher type"),
            };
            println!("{}", result);
        }
    }
}
