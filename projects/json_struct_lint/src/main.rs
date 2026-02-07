use std::env;
use std::fs;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: json_struct_lint <json_file>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents, // Return the content if successful
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let json_value: Value = match serde_json::from_str(&contents) {
        Ok(json_value) => json_value,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            std::process::exit(1);
        }
    };

    if is_valid_structure(&json_value) {
        println!("Valid JSON structure");
    } else {
        println!("Invalid JSON structure");
    }
}

fn is_valid_structure(json: &Value) -> bool {
    if let Some(obj) = json.as_object() {
        // Example structure: Must be an object with 'name' and 'age' keys
        if obj.contains_key("name") && obj.contains_key("age") {
            if obj["name"].is_string() && obj["age"].is_number() {
                return true;
            }
        }
    }
    false
}
