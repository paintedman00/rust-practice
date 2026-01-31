use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <json_file_path> <schema_definition>", args[0]);
        process::exit(1);
    }

    let json_file_path = &args[1];
    let schema_definition = &args[2];

    let contents = match fs::read_to_string(json_file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    };

    let json_value: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            process::exit(1);
        }
    };

    let schema_keys: Vec<&str> = schema_definition.split(',').collect();

    if let Some(json_object) = json_value.as_object() {
        let mut valid = true;
        for key in &schema_keys {
            if !json_object.contains_key(*key) {
                eprintln!("Error: Missing key '{}' in JSON", key);
                valid = false;
            }
        }

        if valid {
            println!("JSON is valid according to the schema.");
        } else {
            process::exit(1);
        }
    } else {
        eprintln!("Error: JSON is not an object.");
        process::exit(1);
    }
}
