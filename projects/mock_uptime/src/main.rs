use std::time::{Duration, Instant};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UptimeResult {
    uptime_seconds: u64,
}

fn main() {
    let mut rng = rand::thread_rng();
    // Simulate uptime between 0 and 86400 seconds (1 day)
    let uptime = rng.gen_range(0..86400);

    let result = UptimeResult {
        uptime_seconds: uptime,
    };

    let json_result = serde_json::to_string(&result).unwrap();

    println!("{}", json_result);
}
