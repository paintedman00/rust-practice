# IP Subnet Calculator

A simple command-line tool to calculate IP subnet information.

## Usage

```bash
# Example usage:
# ip_subnetter 192.168.1.0/24
```

The tool takes an IP address and subnet mask in CIDR notation (e.g., 192.168.1.0/24) as input and outputs relevant subnet information like network address, broadcast address, first host, and last host.

## Building

```bash
cargo build --release
```

## Installing

```bash
cargo install --path .
```
