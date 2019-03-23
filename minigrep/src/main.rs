use std::env;
use std::process;
use minigrep;
use minigrep::Config;

fn main() {
    // let query = &args[1];
    // let filename = &args[2];
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let Config { query, filename } = &config;

    println!("Searching for {}", query);
    println!("In file {}", filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error {}", e);
        process::exit(1);
    }
}