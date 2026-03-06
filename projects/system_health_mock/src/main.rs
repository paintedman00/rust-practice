use rand::Rng;

fn main() {
    let uptime = simulate_uptime();
    let cpu_load = simulate_cpu_load();
    let memory_usage = simulate_memory_usage();

    println!("System Health Report:");
    println!("Uptime: {} seconds", uptime);
    println!("CPU Load: {}%", cpu_load);
    println!("Memory Usage: {}%", memory_usage);

    if cpu_load > 90.0 {
        println!("Warning: High CPU load!");
    }
    if memory_usage > 80.0 {
        println!("Warning: High Memory Usage!");
    }
}

fn simulate_uptime() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(60..86400) // Simulate uptime between 1 minute and 24 hours
}

fn simulate_cpu_load() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(10.0..95.0) // Simulate CPU load between 10% and 95%
}

fn simulate_memory_usage() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(20.0..90.0) // Simulate memory usage between 20% and 90%
}
