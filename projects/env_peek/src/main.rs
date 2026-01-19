use std::collections::HashMap;
use serde::Serialize;
use serde_json;
use std::env;

#[derive(Serialize)]
struct EnvVars {
    vars: HashMap<String, String>,
}

fn main() {
    let mut env_vars: HashMap<String, String> = HashMap::new();

    for (key, value) in env::vars() {
        env_vars.insert(key, value);
    }

    let env_data = EnvVars { vars: env_vars };

    match serde_json::to_string_pretty(&env_data.vars) {
        Ok(json_string) => {
            println!("{}", json_string);
        }
        Err(e) => {
            eprintln!("Error serializing to JSON: {}", e);
        }
    }
}
