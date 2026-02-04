use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: csv_jsonify <input.csv> <output.json>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let mut lines = reader.lines();
    let header = match lines.next() {
        Some(Ok(line)) => line,
        _ => {
            eprintln!("Error: Could not read header from CSV file.");
            std::process::exit(1);
        }
    };

    let headers: Vec<&str> = header.split(',').collect();

    let mut json_output = String::from("[");

    for line in lines {
        let line = line?;
        let values: Vec<&str> = line.split(',').collect();

        if values.len() != headers.len() {
            eprintln!("Warning: Row with inconsistent number of columns.");
            continue;
        }

        let mut row_json = String::from("{");
        for (i, header) in headers.iter().enumerate() {
            row_json.push_str(&format!("\"{}\":\"{}\"", header, values[i]));
            if i < headers.len() - 1 {
                row_json.push(',');
            }
        }
        row_json.push_str("}");
        json_output.push_str(&row_json);
        json_output.push(',');
    }

    if json_output.len() > 1 {
        json_output.pop(); // Remove the last comma
    }

    json_output.push_str("]");

    let mut output_file = File::create(output_path)?;
    output_file.write_all(json_output.as_bytes())?;

    Ok(())
}
