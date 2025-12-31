use clap::{Arg, App};

fn main() {
    let matches = App::new("Simple CLI Tool")
        .version("0.1.0")
        .author("Your Name")
        .about("A simple CLI tool example")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
                .takes_value(true),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let input_file = matches.value_of("input").unwrap_or("default.txt");
    println!("Using input file: {}", input_file);

    println!("Hello, world!");
}
