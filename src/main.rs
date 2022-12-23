use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching \"{}\" in {}",
        config.get_query(),
        config.get_file_path()
    );

    if let Err(err) = minigrep::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}