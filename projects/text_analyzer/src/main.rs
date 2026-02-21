use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "text_analyzer", about = "A simple text analyzer.")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let reader: Box<dyn BufRead> = match opt.input {
        Some(input_path) => {
            let file = File::open(input_path)?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin())), // Read from stdin
    };

    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        lines += 1;
        words += line.split_whitespace().count();
        bytes += line.len() + 1; // +1 for the newline character (assuming Unix-style newlines)
    }

    println!("Lines: {}", lines);
    println!("Words: {}", words);
    println!("Bytes: {}", bytes);

    Ok(())
}
