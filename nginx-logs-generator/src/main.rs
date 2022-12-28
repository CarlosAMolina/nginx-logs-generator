use std::env;
use std::process;

use nginx_logs_generator::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        help();
        process::exit(1);
    });
    if let Err(e) = nginx_logs_generator::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn help() {
    eprintln!(
        "Usage
    cargo run Vec<f32>
        Arguments are the size (Gigabyte) of each log file to generate.
    Example:
        cargo run 1.5 0.5 1"
    )
}
