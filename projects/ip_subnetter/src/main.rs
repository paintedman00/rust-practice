use std::env;
use std::net::Ipv4Addr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ip_subnetter <ip_address>/<cidr>");
        std::process::exit(1);
    }

    let input = &args[1];
    let parts: Vec<&str> = input.split('/').collect();

    if parts.len() != 2 {
        eprintln!("Invalid input format. Use <ip_address>/<cidr>");
        std::process::exit(1);
    }

    let ip_str = parts[0];
    let cidr_str = parts[1];

    let ip: Ipv4Addr = match ip_str.parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IP address: {}", ip_str);
            std::process::exit(1);
        }
    };

    let cidr: u32 = match cidr_str.parse() {
        Ok(cidr) => cidr,
        Err(_) => {
            eprintln!("Invalid CIDR: {}", cidr_str);
            std::process::exit(1);
        }
    };

    if cidr > 32 {
        eprintln!("CIDR must be between 0 and 32");
        std::process::exit(1);
    }

    let mask: u32 = (!0u32).wrapping_shl(32 - cidr);
    let network_address: Ipv4Addr = Ipv4Addr::from(u32::from(ip) & mask);
    let broadcast_address: Ipv4Addr = Ipv4Addr::from(u32::from(ip) | !mask);

    let first_host: Ipv4Addr = Ipv4Addr::from(u32::from(network_address) + 1);
    let last_host: Ipv4Addr = Ipv4Addr::from(u32::from(broadcast_address) - 1);

    println!("Network Address: {}", network_address);
    println!("Broadcast Address: {}", broadcast_address);
    println!("First Host: {}", first_host);
    println!("Last Host: {}", last_host);
}
