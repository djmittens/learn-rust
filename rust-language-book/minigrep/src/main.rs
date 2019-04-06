use minigrep;
use minigrep::Config;
use std::collections::HashMap;
use std::env;
use std::process;

fn main() {
    // let query = &args[1];
    // let filename = &args[2];
    // let args: Vec<String> = env::args().collect();
    let envVars: HashMap<String, String> = env::vars().collect();

    let config = Config::new(env::args(), &envVars).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let Config {
        query,
        filename,
        case_sensitive,
    } = &config;
    // let woo: &mut str = &mut query[0..2];
    // let s: &str = "wooh";

    println!("Searching for {}", query);
    println!("In file {}", filename);
    println!("ignoring case {}", case_sensitive);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}
