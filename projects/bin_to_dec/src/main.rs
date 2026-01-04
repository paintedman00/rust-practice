use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: bin_to_dec <binary_number>");
        process::exit(1);
    }

    let binary_string = &args[1];

    match bin_to_dec(binary_string) {
        Ok(decimal) => println!("{}", decimal),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn bin_to_dec(binary_string: &str) -> Result<i64, String> {
    for c in binary_string.chars() {
        if c != '0' && c != '1' {
            return Err("Invalid binary number: contains non-binary digits.".to_string());
        }
    }

    if binary_string.is_empty() {
        return Err("Invalid binary number: empty string.".to_string());
    }

    let mut decimal: i64 = 0;
    let mut power: i64 = 1;

    for c in binary_string.chars().rev() {
        if c == '1' {
            decimal += power;
        }
        power *= 2;
    }

    Ok(decimal)
}
