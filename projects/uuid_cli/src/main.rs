use rand::Rng;

fn generate_uuid_v4() -> String {
    let mut rng = rand::thread_rng();
    let mut uuid = [0u8; 16];
    rng.fill(&mut uuid[..]);

    // Set version to 4
    uuid[6] = (uuid[6] & 0x0f) | 0x40;

    // Set variant to RFC4122
    uuid[8] = (uuid[8] & 0x3f) | 0x80;

    format!(
        "{:#02x}{:#02x}{:#02x}{:#02x}-{:#02x}{:#02x}-{:#02x}{:#02x}-{:#02x}{:#02x}-{:#02x}{:#02x}{:#02x}{:#02x}{:#02x}{:#02x}",
        uuid[0], uuid[1], uuid[2], uuid[3],
        uuid[4], uuid[5],
        uuid[6], uuid[7],
        uuid[8], uuid[9],
        uuid[10], uuid[11], uuid[12], uuid[13], uuid[14], uuid[15]
    ).replace("0x", "")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let count = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(1)
    } else {
        1
    };

    for _ in 0..count {
        println!("{}", generate_uuid_v4());
    }
}
