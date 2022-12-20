use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Problem parsing arguments");
        help();
        process::exit(1);
    }
    let config = parse_config(&args);
    for file_size in config.files_size.iter() {
        println!("> {}", file_size);
    }
}

struct Config {
    files_size: Vec<f32>,
}

fn parse_config(args: &[String]) -> Config {
    let args_without_script_name = &args[1..];
    let mut files_size = Vec::new();
    for arg in args_without_script_name.iter() {
        // TODO improve error messages
        let file_size = arg.parse::<f32>().expect("Failed to convert argument to float");
        files_size.push(file_size);
    }
    Config { files_size }
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

