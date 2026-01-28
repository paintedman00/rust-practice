use std::time::{Duration, Instant};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    // Simulate system start time.
    let start_time = Instant::now() - Duration::from_secs(rng.gen_range(60..86400)); // Uptime between 1 minute and 1 day

    // Calculate uptime.
    let uptime = Instant::now().duration_since(start_time);

    // Format uptime into human-readable string.
    let uptime_string = format_uptime(uptime);

    println!("System Uptime: {}", uptime_string);
}

fn format_uptime(uptime: Duration) -> String {
    let total_seconds = uptime.as_secs();
    let days = total_seconds / (60 * 60 * 24);
    let hours = (total_seconds % (60 * 60 * 24)) / (60 * 60);
    let minutes = (total_seconds % (60 * 60)) / 60;
    let seconds = total_seconds % 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{:?} day(s)", days));
    }
    if hours > 0 {
        parts.push(format!("{:?} hour(s)", hours));
    }
    if minutes > 0 {
        parts.push(format!("{:?} minute(s)", minutes));
    }
    if seconds > 0 || parts.is_empty() {
        parts.push(format!("{:?} second(s)", seconds));
    }

    parts.join(", ")
}
