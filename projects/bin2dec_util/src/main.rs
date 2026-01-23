use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: bin2dec_util <binary_number>");
        std::process::exit(1);
    }

    let binary_string = &args[1];

    match binary_to_decimal(binary_string) {
        Ok(decimal_value) => {
            println!("{}", decimal_value);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}

fn binary_to_decimal(binary_string: &str) -> Result<u64, String> {
    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return Err("Invalid binary number. Must contain only 0s and 1s.".to_string());
    }

    let mut decimal_value: u64 = 0;
    let mut power: u32 = 0;

    for c in binary_string.chars().rev() {
        if c == '1' {
            decimal_value += 2u64.pow(power);
        }
        power += 1;
    }

    Ok(decimal_value)
}
