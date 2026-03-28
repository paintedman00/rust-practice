use std::fs;
use std::path::PathBuf;
use std::env;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let directory = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    let mut file_sizes: HashMap<u64, u32> = HashMap::new();

    let paths = fs::read_dir(directory)?;

    for entry in paths {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let size = metadata.len();

            *file_sizes.entry(size).or_insert(0) += 1;
        }
    }

    let mut sizes: Vec<u64> = file_sizes.keys().cloned().collect();
    sizes.sort();

    println!("File Size Histogram:");
    for size in sizes {
        println!("Size: {}\tCount: {}", size, file_sizes.get(&size).unwrap());
    }

    Ok(())
}
