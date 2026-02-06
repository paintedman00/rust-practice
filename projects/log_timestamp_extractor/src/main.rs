use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: log_timestamp_extractor <log_file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                // Simple timestamp extraction (adjust regex if needed)
                if let Some(timestamp) = extract_timestamp(&ip) {
                  println!("{}", timestamp);
                }
            }
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_timestamp(line: &str) -> Option<&str> {
    // Very basic timestamp extraction, improve this based on your log format
    // Example: "[2023-10-27 10:00:00] Some log message"
    if line.len() > 20 && line.starts_with('[') && line.contains(']'){
      let end_bracket_index = line.find(']')?;
      return Some(&line[1..end_bracket_index]);
    }
    None
}
