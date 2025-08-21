use colored::*;
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    if config.case_sensitive {

    }

    for line in search(&config.query, &contents){
        for word in line.to_string().split(" ") {
            if word == config.query {
                print!("{}", word.green())
            }
            else{
                print!("{}",word);
            }
            print!(" ");
        }
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(
                "Provide a desired argument: cargo run <search_query> <filename>"
                    .green()
                    .to_string(),
            );
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = if args.len() > 3 && args[3].to_lowercase() == "true" {
            true
        } else {
            false
        };

        return Ok(Config { query, filename,case_sensitive });
    }
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str>{
    let mut res = Vec::new();
    for line in content.lines() {
        if line.contains(query){
            res.push(line);
        }
    }
    res
}
pub fn search_without_case_sensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut res = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query){
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let content: &str = "\
Rust:
safe, fase, productive
Pich three
        ";

        assert_eq!(vec!["safe, fase, productive"], search(query,content));
    }
}