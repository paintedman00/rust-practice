use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: bin_dec <binary_number>");
        process::exit(1);
    }

    let binary_string = &args[1];

    match binary_to_decimal(binary_string) {
        Ok(decimal) => println!("{}", decimal),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn binary_to_decimal(binary_string: &str) -> Result<u64, String> {
    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return Err("Invalid binary number: contains non-binary digits".to_string());
    }

    let mut decimal: u64 = 0;
    let mut power: u32 = 0;

    for c in binary_string.chars().rev() {
        if c == '1' {
            decimal += 2u64.pow(power);
        }
        power += 1;
    }

    Ok(decimal)
}
