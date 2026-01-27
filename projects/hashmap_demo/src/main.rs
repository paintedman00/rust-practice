use std::collections::HashMap;

fn main() {
    // Create a new HashMap
    let mut map: HashMap<String, i32> = HashMap::new();

    // Insert some key-value pairs
    map.insert("Alice".to_string(), 25);
    map.insert("Bob".to_string(), 30);
    map.insert("Charlie".to_string(), 35);

    println!("Initial HashMap: {:?}", map);

    // Retrieve a value by key
    match map.get("Bob") {
        Some(age) => println!("Bob's age: {}", age),
        None => println!("Bob not found in the map."),
    }

    // Try to retrieve a non-existent key
    match map.get("David") {
        Some(age) => println!("David's age: {}", age),
        None => println!("David not found in the map."),
    }

    // Remove a key-value pair
    map.remove("Charlie");
    println!("HashMap after removing Charlie: {:?}", map);

    // Iterate over the HashMap
    println!("Iterating over the HashMap:");
    for (name, age) in &map {
        println!("Name: {}, Age: {}", name, age);
    }

    // Check if a key exists
    if map.contains_key("Alice") {
        println!("Alice is in the map.");
    } else {
        println!("Alice is not in the map.");
    }

    // Get the number of elements
    println!("HashMap size: {}", map.len());
}
