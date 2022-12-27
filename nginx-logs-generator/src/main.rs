use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
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
    let path = Path::new("/tmp/foo.txt");
    let display = path.display();
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    for file_size in config.files_size.iter() {
        println!("> {}", file_size);
        let text_to_write: &str = "foo";
        if let Err(e) = file.write_all(text_to_write.as_bytes()) {
            panic!("couldn't write to {}: {}", display, e);
        }
    }
    println!("successfully wrote to {}", display);
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
