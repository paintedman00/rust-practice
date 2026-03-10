use rand::Rng;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let count = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(1)
    } else {
        1
    };

    for _ in 0..count {
        let uuid = generate_uuid(); 
        println!("{}", uuid);
    }
}

fn generate_uuid() -> String {
    let mut rng = rand::thread_rng();
    let mut uuid = String::new();

    for i in 0..36 {
        if i == 8 || i == 13 || i == 18 || i == 23 {
            uuid.push('-');
        } else {
            let r = rng.gen_range(0..16);
            match r {
                0..=9 => uuid.push((r + 48) as char), // 0-9
                10..=15 => uuid.push((r - 10 + 97) as char), // a-f
                _ => unreachable!(),
            }
        }
    }

    uuid
}
