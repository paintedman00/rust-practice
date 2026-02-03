use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: base64_tool <encode|decode> <string>");
        std::process::exit(1);
    }

    let mode = &args[1];
    let input = &args[2];

    match mode.as_str() {
        "encode" => {
            let encoded = base64::encode(input);
            println!("{}", encoded);
        }
        "decode" => {
            match base64::decode(input) {
                Ok(decoded) => {
                    match String::from_utf8(decoded) {
                        Ok(s) => println!("{}", s),
                        Err(_) => {
                            eprintln!("Error: Invalid UTF-8 sequence");
                            std::process::exit(1);
                        }
                    }
                }
                Err(_) => {
                    eprintln!("Error: Invalid base64 string");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error: Invalid mode. Use 'encode' or 'decode'.");
            std::process::exit(1);
        }
    }
}
