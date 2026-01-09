use std::collections::HashMap;
use serde_json::json;

fn main() {
    let mut env_vars: HashMap<String, String> = HashMap::new();

    for (key, value) in std::env::vars() {
        env_vars.insert(key, value);
    }

    let json_output = json!(env_vars);

    println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
}
