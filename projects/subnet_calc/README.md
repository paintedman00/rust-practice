# Subnet Calculator CLI

A simple command-line interface for calculating subnet information.

## Usage

```bash
cargo run <ip_address>/<cidr_prefix>
```

Example:

```bash
cargo run 192.168.1.10/24
```

## Output

The program outputs information about the subnet, including:

*   IP Address
*   CIDR Prefix
*   Network Address
*   Broadcast Address
*   Usable Host Range
*   Total Hosts
*   Usable Hosts

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```
