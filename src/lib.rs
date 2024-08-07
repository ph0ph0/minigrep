// Import the lib that will allow us to read the contents of a file
use std::fs;
// Import the lib that will allow us to handle errors
use std::error::Error;
// Import the lib that will allow us to env vars passed to the program via terminal
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {

    // `args`  can be any type that implements the Iterator trait and returns a String
    pub fn build(
        mut args: impl Iterator<Item = String>, 
    ) -> Result<Config, &'static str> {
        args.next(); // Skip the first arg which is the path to the program (so we ignore it)

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string provided"),
        };

        let file_path = match args.next() {
           Some(arg) => arg,
           None => return Err("No file path provided"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config { query, file_path, ignore_case })
    }

    // NOTE: Below is the convoluted approach to creating a new instance of Config
    // Above is the more elegant and concise approach
    // Create a new instance of Config
    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    //     // Check if the args passed to the program are less than 3
    //     if args.len() < 3 {
    //         return Err("Not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();
    //     // The below code reads the IGNORE_CASE env var from the terminal
    //     // is_ok checks if the env var is set via the OK variant. If it is set, it returns true
    //     let ignore_case = env::var("IGNORE_CASE").is_ok();

    //     Ok(Config { query, file_path, ignore_case })
    // }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // Read the contents of the file
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_sensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// The lifetime of the returned vector is associated with the lt of the contents
// since we are returning a reference to the contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    // NOTE: Below is the long way of doing the same thing as the code below it
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line.trim());
    //     }
    // }

    contents
        .lines()
        .filter(|l| l.trim().contains(query))
        .map(|l| l.trim())
        .collect()
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // NOTE: See search function
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query.to_lowercase()) {
    //         results.push(line.trim());
    //     }
    // }
    
    // results

    contents
        .lines()
        .filter(|l| 
            l.trim()
            .to_lowercase()
            .contains(&query.trim().to_lowercase())
        )
        .map(|l| l.trim())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        // Search for the string 'duct' in the contents.
        // Search should return the line that this occurs on.
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_sensitive(query, contents));
    }
}