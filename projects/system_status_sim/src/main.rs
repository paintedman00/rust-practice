use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct SystemStatus {
    uptime: String,
    cpu_load: f64,
    memory_usage: f64,
}

fn get_uptime() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let uptime_seconds = now.as_secs();

    let hours = uptime_seconds / 3600;
    let minutes = (uptime_seconds % 3600) / 60;
    let seconds = uptime_seconds % 60;

    format!("{:02}h {:02}m {:02}s", hours, minutes, seconds)
}

fn get_cpu_load() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..100.0)
}

fn get_memory_usage() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..100.0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = SystemStatus {
        uptime: get_uptime(),
        cpu_load: get_cpu_load(),
        memory_usage: get_memory_usage(),
    };

    let json_output = serde_json::to_string_pretty(&status)?;

    println!("{}", json_output);

    Ok(())
}
