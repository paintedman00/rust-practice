use std::collections::HashMap;
use serde_json::json;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: http_response_parser \"HTTP_RESPONSE_STRING\"");
        std::process::exit(1);
    }

    let response_string = &args[1];
    let parsed_response = parse_http_response(response_string);

    match parsed_response {
        Ok(response) => {
            println!("{}", serde_json::to_string_pretty(&response).unwrap());
        }
        Err(e) => {
            eprintln!("Error parsing response: {}", e);
            std::process::exit(1);
        }
    }
}

fn parse_http_response(response_string: &str) -> Result<serde_json::Value, String> {
    let mut parts = response_string.splitn(2, "\r\n\r\n");
    let header_section = parts.next().ok_or("Invalid response format: Missing headers section")?;
    let body = parts.next().unwrap_or("");

    let mut lines = header_section.lines();
    let status_line = lines.next().ok_or("Invalid response format: Missing status line")?.trim();

    let mut headers = HashMap::new();
    for line in lines {
        if line.trim().is_empty() { continue; }
        if let Some((key, value)) = line.split_once(':') {
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Ok(json!({ "status_line": status_line, "headers": headers, "body": body }))
}
