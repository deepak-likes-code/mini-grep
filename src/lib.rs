use colored::Colorize;
use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let result = search(&config.query, &content, config.is_case_sensitive);
    print_result(result, &config.query);
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if ignore_case {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(line);
            }
        } else {
            if line.contains(query) {
                results.push(line);
            }
        }
    }
    results
}

pub fn print_result(result: Vec<&str>, query: &String) {
    if result.len() == 0 {
        println!("No lines found with this {} query ", query.red().bold());
        return;
    } else {
        for lines in result {
            println!("{}", lines.green())
        }
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the file path"),
        };

        let is_case_sensitive = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            is_case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn ignore_case() {
        let query = "rUsT";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Rust:"], search(query, contents, true));
    }
}
