use std::collections::HashMap;

fn main() {
    // Create a new hashmap to store names and ages.
    let mut age_map: HashMap<String, u8> = HashMap::new();

    // Insert some key-value pairs.
    age_map.insert("Alice".to_string(), 30);
    age_map.insert("Bob".to_string(), 25);
    age_map.insert("Charlie".to_string(), 35);

    // Retrieve values by key.
    println!("Alice's age: {:?}", age_map.get("Alice"));
    println!("Bob's age: {:?}", age_map.get("Bob"));
    println!("Charlie's age: {:?}", age_map.get("Charlie"));
    println!("David's age: {:?}", age_map.get("David")); // Key not present

    // Update a value.
    age_map.insert("Alice".to_string(), 31);
    println!("Alice's updated age: {:?}", age_map.get("Alice"));

    // Remove a key-value pair.
    age_map.remove("Bob");
    println!("Bob's age after removal: {:?}", age_map.get("Bob"));

    // Iterate over the hashmap.
    println!("\nAll entries in the hashmap:");
    for (name, age) in &age_map {
        println!("Name: {}, Age: {}", name, age);
    }

    // Check the size of the hashmap
    println!("\nNumber of entries in hashmap: {}", age_map.len());

    // Check if a key exists in the hashmap
    println!("Does Alice exist? {}", age_map.contains_key("Alice"));
    println!("Does Bob exist? {}", age_map.contains_key("Bob"));
}
