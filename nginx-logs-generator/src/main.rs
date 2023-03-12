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
    cargo run String Vec<f32>
        The first argument is the path where the `log` folder will be created to save the log files.
        The next arguments are the size (Gigabyte) of each log file to be generated.
    Example:
        cargo run /tmp 1.5 0.5 1"
    )
}
