use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use chrono::NaiveDateTime;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: log_time_parser <log_file> <timestamp_format>");
        std::process::exit(1);
    }

    let log_file = &args[1];
    let timestamp_format = &args[2];

    let path = Path::new(log_file);

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        // Attempt to parse the timestamp from the beginning of the line
        if let Ok(datetime) = NaiveDateTime::parse_from_str(&line, timestamp_format) {
            println!("{}", datetime);
        } else {
            // If parsing fails, print an error message (optional)
            //eprintln!("Failed to parse timestamp from line: {}", line);
        }
    }

    Ok(())
}
