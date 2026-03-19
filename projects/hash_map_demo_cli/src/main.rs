use std::collections::HashMap;

fn main() {
    // Create a new hash map.
    let mut scores = HashMap::new();

    // Insert some scores.
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    scores.insert("Red", 25);

    // Print the hash map.
    println!("Initial hash map: {:?}", scores);

    // Get a score by key.
    let blue_score = scores.get("Blue");
    match blue_score {
        Some(score) => println!("Blue score: {}", score),
        None => println!("Blue score not found."),
    }

    // Get a score for a non-existent key.
    let green_score = scores.get("Green");
    match green_score {
        Some(score) => println!("Green score: {}", score),
        None => println!("Green score not found."),
    }

    // Update a score.
    scores.insert("Blue", 20);
    println!("Updated hash map: {:?}", scores);

    // Iterate over the hash map.
    println!("Iterating over the hash map:");
    for (team, score) in &scores {
        println!("Team: {}, Score: {}", team, score);
    }

    // Check if a key exists
    if scores.contains_key("Yellow") {
        println!("Yellow team exists!");
    }

    // Remove a key-value pair
    scores.remove("Red");
    println!("Hash map after removing Red: {:?}", scores);
}
