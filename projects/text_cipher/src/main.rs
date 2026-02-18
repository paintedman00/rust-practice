use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 9 {
        eprintln!("Usage: {} --cipher <caesar|xor> --action <encrypt|decrypt> --key <integer|string> --input <text>", args[0]);
        return;
    }

    let cipher_type = &args[2];
    let action = &args[4];
    let key = &args[6];
    let input = &args[8];

    let result = match cipher_type.as_str() {
        "caesar" => {
            let shift = key.parse::<i32>().expect("Invalid Caesar key. Must be an integer.");
            match action.as_str() {
                "encrypt" => caesar_cipher(input, shift),
                "decrypt" => caesar_cipher(input, -shift),
                _ => {
                    eprintln!("Invalid action. Use 'encrypt' or 'decrypt'.");
                    return;
                }
            }
        }
        "xor" => {
            match action.as_str() {
                "encrypt" => xor_cipher(input, key),
                "decrypt" => xor_cipher(input, key),
                _ => {
                    eprintln!("Invalid action. Use 'encrypt' or 'decrypt'.");
                    return;
                }
            }
        }
        _ => {
            eprintln!("Invalid cipher type. Use 'caesar' or 'xor'.");
            return;
        }
    };

    println!("{}", result);
}

fn caesar_cipher(text: &str, shift: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { 'a' as u8 } else { 'A' as u8 };
                let shifted_char = (((c as u8 - base) as i32 + shift).rem_euclid(26) as u8 + base) as char;
                shifted_char
            } else {
                c
            }
        })
        .collect()
}

fn xor_cipher(text: &str, key: &str) -> String {
    let mut result = String::new();
    let key_bytes = key.as_bytes();
    let mut key_index = 0;

    for byte in text.bytes() {
        let xor_byte = byte ^ key_bytes[key_index % key_bytes.len()];
        result.push(xor_byte as char);
        key_index += 1;
    }

    result
}
