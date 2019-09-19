use std::error::Error;
use std::fs::File;
use std::env;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    // w/o using Result
    // pub fn new(args: &[String]) -> Config {
    //     Config {
    //         query: args[1].clone(),
    //         filename: args[2].clone()
    //     }
    // }

    // with using Result
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let matching_lines = if config.case_sensitive {
        println!("Searching case insensitive");
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    if matching_lines.len() > 0 {
        println!("Found {} lines with matching query '{}'", matching_lines.len(), config.query);
    } else {
        println!("No results found!!");
    }

    for line in matching_lines {
        println!("{}", line);
    }

    Ok(())
}

// fn _parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust;
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}