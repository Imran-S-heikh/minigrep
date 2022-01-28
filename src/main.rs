use minigrep::Config;
use std::{env, process};

fn main() -> () {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!(
        "Searching For {}: In ifle {}",
        config.query, config.filename
    );

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");

        process::exit(1);
    }
}
