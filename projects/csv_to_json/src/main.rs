use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: csv_to_json <input.csv> <output.json>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let header_line = match lines.next() {
        Some(Ok(line)) => line,
        _ => {
            eprintln!("Error reading header line");
            std::process::exit(1);
        }
    };
    let headers: Vec<&str> = header_line.split(',').collect();

    let mut json_output = String::from("[");

    for line_result in lines {
        let line = line_result?;
        let values: Vec<&str> = line.split(',').collect();

        if values.len() != headers.len() {
            eprintln!("Warning: Number of values does not match number of headers. Skipping line.");
            continue;
        }

        let mut json_object = String::from("{");

        for (i, header) in headers.iter().enumerate() {
            json_object.push_str(&format!("\"{}\":\"{}\"", header, values[i]));
            if i < headers.len() - 1 {
                json_object.push(',');
            }
        }

        json_object.push('}');
        json_output.push_str(&json_object);
        
        if lines.len_hint().0 > 0 {
            json_output.push(',');
        }
    }

    //Remove the last comma if any element was added to json
    if json_output.len() > 1 { //Check for the '[' character
        if json_output.chars().last().unwrap() == ',' {
            json_output.pop();
        }
    }

    json_output.push(']');

    let mut output = File::create(output_file)?;
    write!(output, "{}")?; 

    Ok(())
}
