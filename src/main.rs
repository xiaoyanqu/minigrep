use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // let query = &args[1];
    // let filename = &args[2];

    // let (query, filename) = parse_config(&args);

    // let config = parse_config(&args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem detected while parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: `{}`", config.query);
    println!("In file: `{}`", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    } else {
        // run(config) returns `()` if it was `Ok(())`, hence here are we.
        // println!("(Woohoo! It works!)");
    };
}
