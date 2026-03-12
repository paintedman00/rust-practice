use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <log_file_path>", args[0]);
        std::process::exit(1);
    }

    let log_file_path = &args[1];

    let (earliest, latest) = analyze_timestamps(log_file_path)?;

    match (earliest, latest) {
        (Some(earliest_ts), Some(latest_ts)) => {
            println!("Earliest Timestamp: {}", earliest_ts);
            println!("Latest Timestamp: {}", latest_ts);
        }
        _ => println!("No timestamps found in the log file."),
    }

    Ok(())
}

fn analyze_timestamps<P: AsRef<Path>>(file_path: P) -> io::Result<(Option<String>, Option<String>)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let timestamp_regex = Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}").unwrap();

    let mut earliest: Option<String> = None;
    let mut latest: Option<String> = None;

    for line in reader.lines() {
        let line = line?;
        if let Some(match_str) = timestamp_regex.find(&line).map(|m| m.as_str().to_string()) {
            if earliest.is_none() || match_str < earliest.clone().unwrap() {
                earliest = Some(match_str.clone());
            }
            if latest.is_none() || match_str > latest.clone().unwrap() {
                latest = Some(match_str);
            }
        }
    }

    Ok((earliest, latest))
}
