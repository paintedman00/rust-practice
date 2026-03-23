use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut status_code: Option<u16> = None;
    let mut content_type: Option<String> = None;

    let mut lines = input.lines();

    if let Some(first_line) = lines.next() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(code) = parts[1].parse::<u16>() {
                status_code = Some(code);
            }
        }
    }

    for line in lines {
        if line.to_lowercase().starts_with("content-type:") {
            content_type = Some(line.split(":").collect::<Vec<&str>>()[1].trim().to_string());
        }
    }

    let output = serde_json::json!({ "status_code": status_code, "content_type": content_type });
    println!("{}", output.to_string());    
}
