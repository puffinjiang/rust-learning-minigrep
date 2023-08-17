use std::{env, process::exit};

use minigrep::{run, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        exit(1);
    });
    println!("Search for {}", config.query);
    println!("In file {}", config.filepath);
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        exit(1);
    }
}
