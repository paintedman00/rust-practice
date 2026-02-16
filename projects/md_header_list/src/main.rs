use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: md_header_list <path_to_markdown_file>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        if line.starts_with('#') {
            println!("{}", line);
        }
    }

    Ok(())
}
