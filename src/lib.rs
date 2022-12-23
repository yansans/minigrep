use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough argument");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
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

    println!("File contents of {}:\n{contents}", config.file_path);

    Ok(())
}
