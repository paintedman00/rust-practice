use std::env;
use std::net::Ipv4Addr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <ip_address>/<cidr_prefix>", args[0]);
        return;
    }

    let input = &args[1];
    let parts: Vec<&str> = input.split('/').collect();

    if parts.len() != 2 {
        eprintln!("Invalid input format. Use <ip_address>/<cidr_prefix>");
        return;
    }

    let ip_str = parts[0];
    let prefix_str = parts[1];

    let ip: Ipv4Addr = match ip_str.parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IP address");
            return;
        }
    };

    let prefix: u8 = match prefix_str.parse() {
        Ok(prefix) => {
            if prefix > 32 {
                eprintln!("Invalid CIDR prefix. Must be between 0 and 32");
                return;
            }
            prefix
        }
        Err(_) => {
            eprintln!("Invalid CIDR prefix");
            return;
        }
    };

    let ip_u32: u32 = u32::from(ip);
    let mask: u32 = !(u32::MAX >> prefix);
    let network_address_u32 = ip_u32 & mask;
    let broadcast_address_u32 = network_address_u32 | !mask;

    let network_address = Ipv4Addr::from(network_address_u32);
    let broadcast_address = Ipv4Addr::from(broadcast_address_u32);

    let total_hosts: u32 = 2u32.pow(u32::from(32 - prefix));
    let usable_hosts: u32 = if prefix < 31 {total_hosts - 2} else {total_hosts};

    println!("IP Address: {}", ip);
    println!("CIDR Prefix: /{}", prefix);
    println!("Network Address: {}", network_address);
    println!("Broadcast Address: {}", broadcast_address);
    println!("Usable Host Range: {} - {}", Ipv4Addr::from(network_address_u32 + 1), Ipv4Addr::from(broadcast_address_u32 - 1));
    println!("Total Hosts: {}", total_hosts);
    println!("Usable Hosts: {}", usable_hosts);
}
