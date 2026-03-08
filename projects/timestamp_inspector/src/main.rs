use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <log_file_path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file `{}`: {}", file_path, err);
        process::exit(1);
    });

    let timestamps = extract_timestamps(&contents);

    let json_output = serde_json::to_string(&timestamps).unwrap(); //Handle errors.
    println!("{}", json_output);
}

fn extract_timestamps(contents: &str) -> Vec<String> {
    let mut timestamps = Vec::new();
    for line in contents.lines() {
        if let Some(timestamp) = extract_timestamp_from_line(line) {
            timestamps.push(timestamp);
        }
    }
    timestamps
}

fn extract_timestamp_from_line(line: &str) -> Option<String> {
    // Simple timestamp pattern: YYYY-MM-DD HH:MM:SS
    if line.len() >= 19 && line.as_bytes()[4] == b'-' && line.as_bytes()[7] == b'-' && line.as_bytes()[10] == b' ' && line.as_bytes()[13] == b':' && line.as_bytes()[16] == b':' {
        let possible_timestamp = &line[0..19];
        // Basic validation to avoid false positives
        if possible_timestamp
            .chars()
            .all(|c| c.is_digit(10) || c == '-' || c == ':' || c == ' ')
        {
            return Some(possible_timestamp.to_string());
        }
    }
    None
}
