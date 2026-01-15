use std::env;
use std::net::Ipv4Addr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <ip_address>/<cidr_prefix>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let parts: Vec<&str> = input.split('/').collect();

    if parts.len() != 2 {
        eprintln!("Invalid input format. Expected <ip_address>/<cidr_prefix>");
        std::process::exit(1);
    }

    let ip_str = parts[0];
    let prefix_str = parts[1];

    let ip: Ipv4Addr = match ip_str.parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IP address: {}", ip_str);
            std::process::exit(1);
        }
    };

    let prefix: u8 = match prefix_str.parse() {
        Ok(prefix) => prefix,
        Err(_) => {
            eprintln!("Invalid CIDR prefix: {}", prefix_str);
            std::process::exit(1);
        }
    };

    if prefix > 32 {
        eprintln!("CIDR prefix must be between 0 and 32");
        std::process::exit(1);
    }

    let ip_u32: u32 = u32::from(ip);
    let mask: u32 = !(u32::MAX >> prefix);
    let network_address_u32 = ip_u32 & mask;
    let broadcast_address_u32 = ip_u32 | !mask;

    let network_address = Ipv4Addr::from(network_address_u32);
    let broadcast_address = Ipv4Addr::from(broadcast_address_u32);

    let num_hosts = (2 as u64).pow(u32::from(32 - prefix)) - 2;

    println!("Network Address: {}", network_address);
    println!("Broadcast Address: {}", broadcast_address);
    println!("Usable Hosts: {}", num_hosts);
}
