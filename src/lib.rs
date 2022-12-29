use colored::Colorize;
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(s) => s,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(s) => s,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    let colored_result = colorize_query(&config.query, result);

    colored_result.iter().for_each(|line| println!("{}", line));

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
}

pub fn colorize_query(query: &str, result: Vec<&str>) -> Vec<String> {
    result
        .iter()
        .map(|line| line.replace(query, &query.yellow()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";

        let test = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, test));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";

        let test = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, test)
        );
    }
}
