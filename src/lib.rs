use std::fs::File;
use std::io::Read;

pub struct Config {
    pub query: String,
    pub file: String
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Missing arguments");
        }

        let file = &args[1].clone();
        let query = &args[2].clone();

        // Create new config
        Ok(Config { query: query.to_string() , file: file.to_string() })
    }
}

pub fn run (config: Config) -> Result<(), &'static str> {
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

pub fn search<'a> ( data: &'a String, query: &'a String ) -> Vec<&'a str> {
    // Vector to append lines containing query
    let mut result = Vec::new();

    // Iterate lines in data, if line contains query, push to vector
    for line in data.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    // Return vector of lines matching query
    result
}