
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("\nNot enough arguements")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
    }
}


pub fn read_file_to_string(filename: &String) -> Result<String, Box<dyn Error>> {
    let file_contents = fs::read_to_string(filename)?;
    Ok(file_contents)
}

pub fn find_string<'a>(query: &str, file_contents: &'a str) -> Option<Vec<&'a str>>  {
    let mut found_strings = Vec::new();

    for line in file_contents.lines() {
        if line.contains(query) {
            found_strings.push(line);
        }
    }
    if found_strings.len() > 0 {
        Some(found_strings)
    }
    else {
        None
    }
}
