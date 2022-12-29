use std::error::Error;
use std::fmt;
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

    let date = Date::new(datetime!(2022 - 01 - 01 00:00:00));
    for file_size in config.files_size.iter() {
        let log = Log::new(date.date);
        println!("> {:?}", log);
        let text_to_write: &str = "foo\n";
        //if let Err(why) = file.write_all(log) {
        if let Err(why) = file.write_all(text_to_write.as_bytes()) {
            let error_msg = format!("couldn't write to {}: {}", display, why);
            return Err(error_msg.into());
        }
    }
    println!("Successfully wrote to {}", display);
    Ok(())
}

struct Date {
    date: time::PrimitiveDateTime,
}

impl Date {
    pub fn new(date: time::PrimitiveDateTime) -> Date {
        Date { date }
    }

    pub fn date(&self) -> time::PrimitiveDateTime {
        self.date
    }

    pub fn add_one_second(&self) -> time::PrimitiveDateTime {
        self.date.saturating_add(1.seconds())
    }

    pub fn date_next_day(&self) -> time::PrimitiveDateTime {
        let mut result = self.date.saturating_add(1.days());
        result = result.replace_hour(0).unwrap();
        result = result.replace_minute(0).unwrap();
        result.replace_second(0).unwrap()
    }
}

//#[derive(Debug, Serialize)]
#[derive(Debug)]
struct Log {
    pub date: time::PrimitiveDateTime,
}

impl Log {
    pub fn new(date: time::PrimitiveDateTime) -> Log {
        Log { date }
    }
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},",
            self.date,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_add_one_second() {
        let date = Date::new(datetime!(2019 - 11 - 26 18:30:59));
        assert_eq!(datetime!(2019 - 11 - 26 18:31:00), date.add_one_second());
    }

    #[test]
    fn date_next_day() {
        let date = Date::new(datetime!(2019 - 11 - 26 18:30:00));
        assert_eq!(datetime!(2019 - 11 - 27 00:00:00), date.date_next_day());
    }
}
