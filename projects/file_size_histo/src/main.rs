use std::fs;
use std::path::PathBuf;
use std::env;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: file_size_histo <directory>");
        std::process::exit(1);
    }

    let directory = &args[1];
    let mut file_sizes: HashMap<String, u32> = HashMap::new();

    let paths = fs::read_dir(directory)?;

    for entry in paths {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let file_size = metadata.len();

            let bucket = match file_size {
                0..=1024 => "0-1KB".to_string(),
                1025..=10240 => "1-10KB".to_string(),
                10241..=102400 => "10-100KB".to_string(),
                102401..=1048576 => "100KB-1MB".to_string(),
                _ => ">1MB".to_string(),
            };

            *file_sizes.entry(bucket).or_insert(0) += 1;
        }
    }

    println!("File Size Histogram:");
    let categories = vec!["0-1KB", "1-10KB", "10-100KB", "100KB-1MB", ">1MB"];
    for category in categories {
        let count = file_sizes.get(category).unwrap_or(&0);
        let stars = "*".repeat(*count as usize);
        println!("{}:   {}", category, stars);
    }

    Ok(())
}
