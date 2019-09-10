use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for '{}'", config.query);
    println!("In filename '{}'", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("something went wrong reading the file.");

    println!("file text {}", contents);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        Config {
            query: args[1].clone(),
            filename: args[2].clone()
        }
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
