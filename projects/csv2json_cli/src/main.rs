use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: csv2json_cli <input.csv> <output.json>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let input_path = Path::new(input_file);
    let output_path = Path::new(output_file);

    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let headers = match lines.next() {
        Some(Ok(line)) => line.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>(),
        _ => {
            eprintln!("Error: Could not read headers from CSV");
            std::process::exit(1);
        }
    };

    let mut json_output = String::from("[");
    let mut first_record = true;

    for line in lines {
        let line = line?;
        let values = line.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>();

        if values.len() != headers.len() {
            eprintln!("Warning: Skipping line due to inconsistent number of columns");
            continue;
        }

        if !first_record {
            json_output.push_str(",");
        } else {
            first_record = false;
        }

        json_output.push_str("{");
        let mut first_field = true;
        for (i, header) in headers.iter().enumerate() {
            if !first_field {
                json_output.push_str(",");
            } else {
                first_field = false;
            }
            json_output.push_str(&format!("\"{}\": \"{}\"", header, values[i]));
        }
        json_output.push_str("}");
    }

    json_output.push_str("]");

    let mut output = File::create(output_path)?;
    output.write_all(json_output.as_bytes())?;

    Ok(())
}
