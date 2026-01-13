use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <value> <from_unit> <to_unit>", args[0]);
        process::exit(1);
    }

    let value: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid temperature value.");
            process::exit(1);
        }
    };

    let from_unit = args[2].to_uppercase();
    let to_unit = args[3].to_uppercase();

    let result = match (from_unit.as_str(), to_unit.as_str()) {
        ("C", "F") => celsius_to_fahrenheit(value),
        ("C", "K") => celsius_to_kelvin(value),
        ("F", "C") => fahrenheit_to_celsius(value),
        ("F", "K") => fahrenheit_to_kelvin(value),
        ("K", "C") => kelvin_to_celsius(value),
        ("K", "F") => kelvin_to_fahrenheit(value),
        _ => {
            eprintln!("Error: Invalid unit conversion.");
            process::exit(1);
        }
    };

    println!("{}", result);
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    celsius_to_kelvin(fahrenheit_to_celsius(fahrenheit))
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    celsius_to_fahrenheit(kelvin_to_celsius(kelvin))
}
