use std::env;
use std::process;
use std::fs::File;
use std::io::Read;

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
    match run(config) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    };
}

struct Config {
    query: String,
    file: String
}

impl Config {
    fn new (args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Missing arguments");
        }

        let file = &args[1].clone();
        let query = &args[2].clone();

        Ok(Config { query: query.to_string() , file: file.to_string() })
    }
}

fn run (config: Config) -> Result<(), &'static str> {
    // Open file
    let mut file = match File::open(config.file) {
        Ok(f) => f,
        Err(_) => return Err("File not found")
    };

    // Dump file contents
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(s) => s,
        Err(_) => return Err("Cannot open file")
    };

    // Search data
    let data = search(&data, &config.query);

    // Print data
    if data.len() < 1 { println!("[No results containing {}]", config.query) };
    
    for line in data {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a> ( data: &'a String, query: &'a String ) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in data.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}