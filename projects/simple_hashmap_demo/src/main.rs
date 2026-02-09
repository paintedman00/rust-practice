use std::collections::HashMap;
use std::io;

fn main() {
    let mut map: HashMap<String, String> = HashMap::new();

    println!("Enter key-value pairs (enter empty key to finish):");

    loop {
        let mut key = String::new();
        print!("Enter key: ");
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut key).expect("Failed to read line");
        let key = key.trim();

        if key.is_empty() {
            break;
        }

        let mut value = String::new();
        print!("Enter value: ");
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut value).expect("Failed to read line");
        let value = value.trim();

        map.insert(key.to_string(), value.to_string());
    }

    println!("\nEnter key to retrieve (enter empty key to finish):");

    loop {
        let mut key = String::new();
        print!("Enter key to retrieve: ");
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut key).expect("Failed to read line");
        let key = key.trim();

        if key.is_empty() {
            break;
        }

        match map.get(key) {
            Some(value) => println!("Value for key '{}': {}", key, value),
            None => println!("Key '{}' not found.", key),
        }
    }
}
