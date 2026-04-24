use std::error::Error;
use std::env;
use std::fs;

pub const USAGE: &str = "Usage: minigrep <query> <file_path>";

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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

        if args.next().is_some() {
            return Err("too many arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    println!("Case insensitive: {}", config.ignore_case);
    println!("Matches found: {}", results.len());

    if results.is_empty() {
        println!("No matches found.");
    } else {
        println!();
        for line in results {
            println!("{line}");
        }
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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

    #[test]
    fn case_sensitive_search_distinguishes_case() {
        let query = "rUsT";
        let contents = "\
Rust:
Trust me.";

        assert!(search(query, contents).is_empty());
    }

    #[test]
    fn case_insensitive_search_finds_mixed_case() {
        let query = "rUsT";
        let contents = "\
Rust:
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

    #[test]
    fn build_rejects_extra_arguments() {
        let args = vec![
            "minigrep".to_string(),
            "query".to_string(),
            "file.txt".to_string(),
            "extra".to_string(),
        ];

        match Config::build(args.into_iter()) {
            Ok(_) => panic!("expected parse failure"),
            Err(err) => assert_eq!("too many arguments", err),
        }
    }

    #[test]
    fn build_accepts_exact_arguments() {
        let args = vec![
            "minigrep".to_string(),
            "query".to_string(),
            "file.txt".to_string(),
        ];

        let config = Config::build(args.into_iter()).expect("expected valid config");

        assert_eq!("query", config.query);
        assert_eq!("file.txt", config.file_path);
    }
}
