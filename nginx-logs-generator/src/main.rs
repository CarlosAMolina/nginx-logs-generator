use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        help();
        process::exit(1);
    });
    for file_size in config.files_size.iter() {
        println!("> {}", file_size);
    }
}

struct Config {
    files_size: Vec<f32>,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let args_without_script_name = &args[1..];
        let mut files_size = Vec::new();
        for arg in args_without_script_name.iter() {
            // TODO improve error messages
            let error_msg = format!("Argument `{}` cannot be converted to float", arg);
            let file_size = arg.parse::<f32>().expect(&error_msg);
            files_size.push(file_size);
        }
        Ok(Config { files_size })
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
