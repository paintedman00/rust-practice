use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <log_file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let re = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}").unwrap();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                for mat in re.find_iter(&ip) {
                    println!("{}", mat.as_str());
                }
            }
        }
    }

    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
