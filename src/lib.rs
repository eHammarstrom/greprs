extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

use regex::Regex;

pub struct Config<'a> {
    pub search: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let search = &args[1];
        let filename = &args[2];

        let mut case_sensitive = true;

        for (name, _) in env::vars() {
            if name == "CASE_INSENSITIVE" {
                case_sensitive = false;
            }
        }

        Ok(Config {
            search: search,
            filename: filename,
            case_sensitive: case_sensitive
        })
    }
}

fn grep<'a>(search: &str, contents: &'a str) -> Result<Vec<&'a str>, Box<Error>> {
    let mut results = Vec::new();
    let re = Regex::new(search)?;

    for line in contents.lines() {
        if re.is_match(line) {
            results.push(line);
        }
    }

    Ok(results)
}

fn grep_case_insensitive<'a>(search: &str, contents: &'a str) -> Result<Vec<&'a str>, Box<Error>> {
    let search = &search.to_lowercase();
    let mut results = Vec::new();
    let re = Regex::new(search)?;

    for line in contents.lines() {
        if re.is_match(&line.to_lowercase()) {
            results.push(line);
        }
    }

    Ok(results)
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        grep(&config.search, &contents)?
    } else {
        grep_case_insensitive(&config.search, &contents)?
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use {grep, grep_case_insensitive};

    #[test]
    fn case_sensitive() {
        let search = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(
            vec!["safe, fast, productive."],
            grep(search, contents).unwrap()
        );
    }

    #[test]
    fn case_insensitive() {
        let search = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            grep_case_insensitive(search, contents).unwrap()
        );
    }
}