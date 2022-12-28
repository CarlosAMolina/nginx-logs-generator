use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use time::ext::NumericalDuration;
use time::macros::datetime;

pub struct Config {
    pub files_size: Vec<f32>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = Path::new("/tmp/foo.txt");
    let display = path.display();
    let mut file = match File::create(path) {
        Err(why) => {
            let error_msg = format!("couldn't create {}: {}", display, why);
            return Err(error_msg.into());
        }
        Ok(file) => file,
    };
    for file_size in config.files_size.iter() {
        println!("> {}", file_size);
        let text_to_write: &str = "foo\n";
        if let Err(why) = file.write_all(text_to_write.as_bytes()) {
            let error_msg = format!("couldn't write to {}: {}", display, why);
            return Err(error_msg.into());
        }
    }
    println!("Successfully wrote to {}", display);
    Ok(())
}

fn add_one_second(date: time::PrimitiveDateTime) -> time::PrimitiveDateTime {
    datetime!(2019 - 11 - 26 18:31:01)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increase_date() {
        let date = datetime!(2019 - 11 - 26 18:30:59);
        assert_eq!(datetime!(2019 - 11 - 26 18:31:00), add_one_second(date));
    }
}
