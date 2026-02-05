use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: crypt_cli <mode> <key> <input_string>");
        std::process::exit(1);
    }

    let mode = &args[1];
    let key_str = &args[2];
    let input_string = &args[3];

    match mode.as_str() {
        "caesar" => {
            let shift: i32 = key_str.parse().unwrap_or_else(|_| {
                eprintln!("Invalid Caesar shift. Must be an integer.");
                std::process::exit(1);
            });
            let result = caesar_cipher(input_string, shift);
            println!("{}", result);
        }
        "xor" => {
            let key = u8::from_str_radix(key_str, 16).unwrap_or_else(|_| {
                eprintln!("Invalid XOR key. Must be a hexadecimal value.");
                std::process::exit(1);
            });
            let result = xor_cipher(input_string, key);
            println!("{}", result);
        }
        _ => {
            eprintln!("Invalid mode. Choose 'caesar' or 'xor'.");
            std::process::exit(1);
        }
    }
}

fn caesar_cipher(text: &str, shift: i32) -> String {
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

fn xor_cipher(text: &str, key: u8) -> String {
    text.bytes()
        .map(|b| (b ^ key) as char)
        .collect()
}
