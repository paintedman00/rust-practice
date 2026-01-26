use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;
    let mut filename = String::new();

    let reader: Box<dyn BufRead> = if args.len() > 1 {
        filename = args[1].clone();
        let file = File::open(&filename)?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    for line_result in reader.lines() {
        let line = line_result?;
        lines += 1;
        words += line.split_whitespace().count();
        chars += line.chars().count();
    }

    if !filename.is_empty() {
        println!("{}\t{}\t{}\t{}", lines, words, chars, filename);
    } else {
        println!("{}\t{}\t{}", lines, words, chars);
    }

    Ok(())
}
