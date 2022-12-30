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
    let mut date = Date::new(datetime!(2022 - 01 - 01 00:00:00));
    const LINES_TO_WRITE_IN_EACH_CHECK: i32 = 500;
    for file_size in config.files_size.iter() {
        for _ in 0..LINES_TO_WRITE_IN_EACH_CHECK {
            let log = Log::new(date.date);
            let mut text_to_write = log.str();
            text_to_write.push_str("\n");
            if let Err(why) = file.write_all(text_to_write.as_bytes()) {
                let error_msg = format!("couldn't write to {}: {}", display, why);
                return Err(error_msg.into());
            }
            date.add_one_second();
        }
        date.set_next_day();
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

    pub fn add_one_second(&mut self) {
        self.date = self.date.saturating_add(1.seconds());
    }

    pub fn set_next_day(&mut self) {
        self.date = self.date.saturating_add(1.days());
        self.date = self.date.replace_hour(0).unwrap();
        self.date = self.date.replace_minute(0).unwrap();
        self.date = self.date.replace_second(0).unwrap();
    }
}

#[derive(Debug)]
struct Log {
    pub date: time::PrimitiveDateTime,
}

impl Log {
    pub fn new(date: time::PrimitiveDateTime) -> Log {
        Log { date }
    }

    pub fn str(&self) -> String {
        format!(
            r#"{} - {} {} "{}" {} {} "{}" "{}""#,
            self.remote_addr(),
            self.remote_user(),
            self.time_local(),
            self.request(),
            self.status(),
            self.body_bytes_sent(),
            self.http_referer(),
            self.http_user_agent(),
        )
    }

    fn body_bytes_sent(&self) -> String {
        "118".to_string()
    }

    fn http_referer(&self) -> String {
        "http://foo-referer/login.asp".to_string()
    }

    fn http_user_agent(&self) -> String {
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:71.0) Gecko/20100101 Firefox/71.0".to_string()
    }

    fn remote_addr(&self) -> String {
        "8.8.8.8".to_string()
    }

    fn request(&self) -> String {
        "GET /index.html HTTP/1.1".to_string()
    }

    fn remote_user(&self) -> String {
        "-".to_string()
    }

    fn status(&self) -> String {
        "200".to_string()
    }

    fn time_local(&self) -> String {
        let format = time::macros::format_description!(
            "[[[day]/[month repr:short]/[year]:[hour]:[minute]:[second] +0100]"
        );
        self.date.format(&format).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_add_one_second_twice() {
        let mut date = Date::new(datetime!(2019 - 11 - 26 18:30:59));
        date.add_one_second();
        assert_eq!(datetime!(2019 - 11 - 26 18:31:00), date.date());
        date.add_one_second();
        assert_eq!(datetime!(2019 - 11 - 26 18:31:01), date.date());
    }

    #[test]
    fn date_set_next_day_twice() {
        let mut date = Date::new(datetime!(2019 - 11 - 26 18:30:00));
        date.set_next_day();

        assert_eq!(datetime!(2019 - 11 - 27 00:00:00), date.date());
        date.set_next_day();
        assert_eq!(datetime!(2019 - 11 - 28 00:00:00), date.date());
    }

    #[test]
    fn date_add_one_second_and_set_next_day_update_values_correctly() {
        let mut date = Date::new(datetime!(2019 - 11 - 26 18:30:20));

        date.add_one_second();
        date.set_next_day();
        assert_eq!(datetime!(2019 - 11 - 27 00:00:00), date.date());
    }

    #[test]
    fn log_has_correct_format() {
        let date = Date::new(datetime!(2021 - 12 - 16 00:07:02));
        let log = Log::new(date.date);
        assert_eq!(
            r#"8.8.8.8 - - [16/Dec/2021:00:07:02 +0100] "GET /index.html HTTP/1.1" 200 118 "http://foo-referer/login.asp" "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:71.0) Gecko/20100101 Firefox/71.0""#,
            log.str()
        );
    }
}
