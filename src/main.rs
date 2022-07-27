use std::env;
use std::process;

use cliexample::Config;
use cliexample::read_file_to_string;
use cliexample::find_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    // get the args parsed from parse_args
    let grep_config = Config::new(&args).unwrap_or_else(|err| {
        println!("\nNot able to parse args {}", err);
        process::exit(1);
    });

    
    // get the file contents
    let file_contents = read_file_to_string(&grep_config.filename).unwrap_or_else(|err| {
        println!("\nNot able to parse args {}", err);
         process::exit(1);
    });

    // get the results
    match find_string(&grep_config.query, &file_contents) {
        Some(results) => {
            for line in results {
                println!("{}\n", line);
            }
        },
        None => {
            println!("\n No matches found!");
        }
    }
    
}
