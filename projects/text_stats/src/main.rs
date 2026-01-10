use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;

    let reader: Box<dyn BufRead> = if args.len() > 1 {
        let filename = &args[1];
        let file = File::open(filename)?;
        Box::new(io::BufReader::new(file))
    } else {
        Box::new(io::BufReader::new(io::stdin()))
    };

    for line_result in reader.lines() {
        let line = line_result?;
        lines += 1;
        chars += line.chars().count();
        words += line.split_whitespace().count();
    }

    println!("Lines: {}", lines);
    println!("Words: {}", words);
    println!("Characters: {}", chars);

    Ok(())
}
