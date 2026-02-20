use std::collections::HashMap;

fn main() {
    // Create a new HashMap to store string keys and integer values.
    let mut my_map: HashMap<String, i32> = HashMap::new();

    // Insert some key-value pairs.
    my_map.insert("apple".to_string(), 1);
    my_map.insert("banana".to_string(), 2);
    my_map.insert("cherry".to_string(), 3);

    println!("HashMap contents: {:?}", my_map);

    // Retrieve a value based on a key.
    let value = my_map.get("banana");
    match value {
        Some(v) => println!("The value associated with 'banana' is: {}", v),
        None => println!("The key 'banana' is not found."),
    }

    // Check if a key exists.
    if my_map.contains_key("apple") {
        println!("The key 'apple' exists in the HashMap.");
    } else {
        println!("The key 'apple' does not exist in the HashMap.");
    }

    // Remove a key-value pair.
    my_map.remove("banana");

    println!("HashMap contents after removing 'banana': {:?}", my_map);

    // Retrieve a value after removing the key
    let value = my_map.get("banana");
    match value {
        Some(v) => println!("The value associated with 'banana' is: {}", v),
        None => println!("The key 'banana' is not found."),
    }
}
