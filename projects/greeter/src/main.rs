use clap::{Arg, App};

fn main() {
    let matches = App::new("Greeter")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("Greets the user.")
        .arg(Arg::with_name("name")
             .short("n")
             .long("name")
             .value_name("NAME")
             .help("Sets the name of the person to greet")
             .required(true)
             .takes_value(true))
        .get_matches();

    let name = matches.value_of("name").unwrap();

    println!("Hello, {}!", name);
}
