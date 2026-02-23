use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: http_response_peek <http_response_string>");
        process::exit(1);
    }

    let response_string = &args[1];

    let mut parts = response_string.splitn(2, "\r\n\r\n");
    let header_block = parts.next().unwrap_or("");
    let body = parts.next().unwrap_or("");

    let mut headers = header_block.split("\r\n");
    let status_line = headers.next().unwrap_or("");

    let mut header_map = std::collections::HashMap::new();
    for header in headers {
        if let Some(colon_index) = header.find(':') {
            let name = header[..colon_index].trim();
            let value = header[colon_index + 1..].trim();
            header_map.insert(name.to_string(), value.to_string());
        }
    }

    let json_output = serde_json::json!({ 
        "status_line": status_line, 
        "headers": header_map, 
        "body": body 
    });

    println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
}
