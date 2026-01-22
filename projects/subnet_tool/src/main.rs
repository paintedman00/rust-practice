use std::net::Ipv4Addr;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SubnetInfo {
    network_address: String,
    first_usable_ip: String,
    last_usable_ip: String,
    broadcast_address: String,
    total_hosts: u32,
    usable_hosts: u32,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: subnet_tool <ip_address> <cidr_prefix>");
        std::process::exit(1);
    }

    let ip_address = &args[1];
    let cidr_prefix = args[2].parse::<u8>().unwrap_or_else(|_| {
        eprintln!("Invalid CIDR prefix. Must be a number between 0 and 32.");
        std::process::exit(1);
    });

    if cidr_prefix > 32 {
        eprintln!("Invalid CIDR prefix. Must be between 0 and 32.");
        std::process::exit(1);
    }

    let ip = Ipv4Addr::from_str(ip_address).unwrap_or_else(|_| {
        eprintln!("Invalid IP address.");
        std::process::exit(1);
    });

    let ip_u32: u32 = ip.into();
    let mask: u32 = (!0u32).wrapping_shl(cidr_prefix as u32);
    let network_address_u32 = ip_u32 & mask;
    let broadcast_address_u32 = network_address_u32 | !mask;

    let network_address = Ipv4Addr::from(network_address_u32).to_string();
    let broadcast_address = Ipv4Addr::from(broadcast_address_u32).to_string();
    let first_usable_ip = Ipv4Addr::from(network_address_u32.wrapping_add(1)).to_string();
    let last_usable_ip = Ipv4Addr::from(broadcast_address_u32.wrapping_sub(1)).to_string();

    let total_hosts: u32 = 2u32.pow(32 - cidr_prefix as u32);
    let usable_hosts: u32 = if total_hosts > 2 { total_hosts - 2 } else { 0 };

    let subnet_info = SubnetInfo {
        network_address,
        first_usable_ip,
        last_usable_ip,
        broadcast_address,
        total_hosts,
        usable_hosts,
    };

    let json_output = serde_json::to_string(&subnet_info).unwrap();
    println!("{}", json_output);
}
