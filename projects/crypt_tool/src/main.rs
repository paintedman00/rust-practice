use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut algorithm = "caesar";
    let mut action = "encrypt";
    let mut key: i32 = 0;
    let mut input = "";

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--algorithm" => {
                i += 1;
                if i < args.len() {
                    algorithm = &args[i];
                }
            }
            "--action" => {
                i += 1;
                if i < args.len() {
                    action = &args[i];
                }
            }
            "--key" => {
                i += 1;
                if i < args.len() {
                    key = args[i].parse::<i32>().unwrap_or(0);
                }
            }
            "--input" => {
                i += 1;
                if i < args.len() {
                    input = &args[i];
                }
            }
            _ => (),
        }
        i += 1;
    }

    if key == 0 || input.is_empty() {
        println!("Usage: crypt_tool --algorithm <caesar|xor> --action <encrypt|decrypt> --key <integer> --input <text>");
        return;
    }

    let result = match (algorithm, action) {
        ("caesar", "encrypt") => caesar(input, key),
        ("caesar", "decrypt") => caesar(input, -key),
        ("xor", "encrypt") => xor(input, key),
        ("xor", "decrypt") => xor(input, key),
        _ => String::from("Invalid algorithm or action"),
    };

    println!("{}", result);
}

fn caesar(text: &str, key: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted_char = (((c as u8 - base) as i32 + key).rem_euclid(26) as u8 + base) as char;
                shifted_char
            } else {
                c
            }
        })
        .collect()
}

fn xor(text: &str, key: i32) -> String {
    text.chars()
        .map(|c| (c as u8 ^ key as u8) as char)
        .collect()
}
