use std::env;
use std::fs;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: json_validator <schema_file> <json_file>");
        std::process::exit(1);
    }

    let schema_file = &args[1];
    let json_file = &args[2];

    let schema_content = match fs::read_to_string(schema_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading schema file: {}", e);
            std::process::exit(1);
        }
    };

    let json_content = match fs::read_to_string(json_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading JSON file: {}", e);
            std::process::exit(1);
        }
    };

    let schema: Value = match serde_json::from_str(&schema_content) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error parsing schema JSON: {}", e);
            std::process::exit(1);
        }
    };

    let json: Value = match serde_json::from_str(&json_content) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error parsing JSON file: {}", e);
            std::process::exit(1);
        }
    };

    // A basic validation example.  More robust validation would require a proper schema validator.
    if let Value::Object(schema_obj) = &schema {
        if let Value::Object(json_obj) = &json {
            if let Some(Value::Array(required_fields)) = schema_obj.get("required") {
                for field in required_fields {
                    if let Value::String(field_name) = field {
                        if !json_obj.contains_key(field_name) {
                            println!("Invalid JSON: Missing required field '{}'", field_name);
                            std::process::exit(1);
                        }
                    }
                }
            }
        } else {
            println!("Invalid JSON: Input is not a JSON object");
            std::process::exit(1);
        }
    } else {
        println!("Invalid JSON: Schema is not a JSON object");
        std::process::exit(1);
    }

    println!("Valid JSON");
}
