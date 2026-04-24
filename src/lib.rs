use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // First argument is the executable name, so we discard it.
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments"),
        };

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let results = search(&config.query, &contents);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    println!();

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_single_matching_line() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn finds_multiple_matching_lines() {
        let query = "bo";
        let contents = "\
book
rust
robot
boat";

        assert_eq!(vec!["book", "robot", "boat"], search(query, contents));
    }

    #[test]
    fn returns_empty_when_no_match() {
        let query = "zzz";
        let contents = "\
alpha
beta
gamma";

        assert!(search(query, contents).is_empty());
    }
}
