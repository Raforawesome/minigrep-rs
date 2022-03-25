use minigrep::Config;
use std::env;

fn main() -> () {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("Fatal error: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}

