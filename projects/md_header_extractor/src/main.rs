use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let path = Path::new(filename);
    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            let level = line.chars().take_while(|&c| c == '#').count();
            let header_text = line[level..].trim();
            println!("{}", header_text);
        }
    }

    Ok(())
}
