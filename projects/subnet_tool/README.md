# Subnet Tool

A simple command-line tool for calculating subnet information.

## Usage

```bash
subnet_tool <ip_address> <cidr_prefix>
```

Example:

```bash
subnet_tool 192.168.1.10 24
```

## Output

The tool will output the following information in JSON format:

*   Network Address
*   First Usable IP
*   Last Usable IP
*   Broadcast Address
*   Total Hosts
*   Usable Hosts

## Building

```bash
cargo build --release
```
