use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        help();
        process::exit(1);
    });
    run(config);
}

fn run(config: Config) {
    for file_size in config.files_size.iter() {
        println!("> {}", file_size);
    }
}

struct Config {
    files_size: Vec<f32>,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err("not enough arguments".to_string());
        }
        let args_without_script_name = &args[1..];
        let mut files_size = Vec::new();
        for arg in args_without_script_name.iter() {
            let mut error_msg = format!("argument `{}` cannot be parsed", arg);
            let arg_parsed = arg.parse::<f32>();
            match arg_parsed {
                Ok(file_size) => {
                    if file_size <= 0.0 {
                        error_msg = format!("{}, it must be greater than 0", error_msg);
                        return Err(error_msg);
                    }
                    files_size.push(file_size);
                }
                Err(_) => {
                    let error_msg = format!("{}, cannot be converted to float", error_msg);
                    return Err(error_msg);
                }
            }
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
