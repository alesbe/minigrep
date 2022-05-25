use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Take args
    let args: Vec<String> = env::args().collect();

    // Create a new config
    let config = match Config::new(&args) {
        Ok(c) => {
            println!("Searching {} in file {}\n", c.file, c.query);
            c
        },
        Err(err) => {
            println!("Error: {}", err);
            process::exit(1);
        }
    };

    // Run minigrep 
    match minigrep::run(config) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    };
}