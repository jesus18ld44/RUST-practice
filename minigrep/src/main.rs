use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // unwrap_or_else() is defined on Result<T, E>
    // If Ok, similar to unwrap()
    // If Err, calls the code in the closurem which is an anonymous function we define and pass as
    // ann argument to unwrap_or_else()
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
// 
//     Config { query, filename }
// }
