#![allow(unused_imports)]
use flate2::write::GzEncoder;
use flate2::Compression;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::fs;
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
    let mut date = Date::new(datetime!(2022 - 01 - 01 00:00:00));
    let number_of_files_to_create: u8 = config.files_size.len().try_into().unwrap();
    let mut file_name_generator = FileNameGenerator::new(number_of_files_to_create);
    let folder_path_name = "/tmp/logs";
    if Path::new(folder_path_name).exists() {
        fs::remove_dir_all(folder_path_name)?;
    }
    fs::create_dir_all(folder_path_name)?;
    for file_size_to_create in config.files_size.iter() {
        let file_path_name = format!("{}/{}", folder_path_name, file_name_generator.name());
        let path = Path::new(&file_path_name);
        let display = path.display();
        let mut file = match File::create(path) {
            Err(why) => {
                let error_msg = format!("couldn't create {}: {}", display, why);
                return Err(error_msg.into());
            }
            Ok(file) => file,
        };
        let file_size_bytes = get_bytes_from_gigabytes(*file_size_to_create);
        let number_of_logs_to_write = get_number_of_logs_to_write(file_size_bytes);
        println!(
            "Creating file of {:?} GB, writing {:?} logs",
            file_size_to_create, number_of_logs_to_write
        );
        for _ in 0..number_of_logs_to_write {
            let log = Log::new(date.date);
            let mut text_to_write = log.str();
            text_to_write.push('\n');
            if let Err(why) = file.write_all(text_to_write.as_bytes()) {
                let error_msg = format!("couldn't write to {}: {}", display, why);
                return Err(error_msg.into());
            }
            date.add_one_second();
        }
        let file_size_bytes_created = get_file_size_bytes(&file_path_name).unwrap();
        if file_size_bytes_created < file_size_bytes {
            let error_msg = format!(
                "couldn't create a file of {} bytes, {} bytes have been created",
                file_size_bytes, file_size_bytes_created
            );
            return Err(error_msg.into());
        } else {
            println!(
                "The file `{}` of {} bytes has been created",
                display, file_size_bytes_created
            );
        }
        let file_compressor = FileCompressor::new(file_path_name);
        if file_compressor.must_compress_the_file() {
            println!("Compressing the file");
            file_compressor.compress_file_as_gz()?;
            file_compressor.remove_original_file()?;
        }
        date.set_next_day();
    }
    Ok(())
}

struct Date {
    date: time::PrimitiveDateTime,
}

impl Date {
    pub fn new(date: time::PrimitiveDateTime) -> Date {
        Date { date }
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

    #[cfg(test)]
    fn body_bytes_sent(&self) -> u16 {
        118
    }
    #[cfg(not(test))]
    fn body_bytes_sent(&self) -> u32 {
        let choices = vec![77, 118, 150, 361, 125837];
        self.get_random_vector_element(choices)
    }

    #[allow(dead_code)]
    fn get_random_vector_element<T: Clone>(&self, vector: Vec<T>) -> T {
        let mut rng = thread_rng();
        let result = vector.choose(&mut rng).unwrap();
        result.clone()
    }

    #[cfg(test)]
    fn http_referer(&self) -> String {
        "http://foo-referer/login.asp".to_string()
    }
    #[cfg(not(test))]
    fn http_referer(&self) -> String {
        let choices = vec!["-".to_string(), "http://foo-referer/login.asp".to_string()];
        self.get_random_vector_element(choices)
    }

    #[cfg(test)]
    fn http_user_agent(&self) -> String {
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:71.0) Gecko/20100101 Firefox/71.0".to_string()
    }
    #[cfg(not(test))]
    fn http_user_agent(&self) -> String {
        let choices = vec![
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:71.0) Gecko/20100101 Firefox/71.0".to_string(),
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.67 Safari/537.36".to_string(),
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36".to_string(),
        ];
        self.get_random_vector_element(choices)
    }

    #[cfg(test)]
    fn remote_addr(&self) -> String {
        "8.8.8.8".to_string()
    }
    #[cfg(not(test))]
    fn remote_addr(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.get_random_ip_v4_element(),
            self.get_random_ip_v4_element(),
            self.get_random_ip_v4_element(),
            self.get_random_ip_v4_element(),
        )
    }

    #[allow(dead_code)]
    fn get_random_ip_v4_element(&self) -> u16 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..256)
    }

    #[cfg(test)]
    fn request(&self) -> String {
        "GET /index.html HTTP/1.1".to_string()
    }
    #[cfg(not(test))]
    fn request(&self) -> String {
        let choices = vec![
            "GET / HTTP/1.1".to_string(),
            "GET /index.html HTTP/1.1".to_string(),
            "POST /foo/admin/formLogin HTTP/1.1".to_string(),
        ];
        self.get_random_vector_element(choices)
    }

    #[cfg(test)]
    fn remote_user(&self) -> String {
        "-".to_string()
    }
    #[cfg(not(test))]
    fn remote_user(&self) -> String {
        let choices = vec!["-".to_string(), "root".to_string()];
        self.get_random_vector_element(choices)
    }

    #[cfg(test)]
    fn status(&self) -> u16 {
        200
    }
    #[cfg(not(test))]
    fn status(&self) -> u16 {
        let choices = vec![200, 301, 400, 404, 405];
        self.get_random_vector_element(choices)
    }

    fn time_local(&self) -> String {
        let format = time::macros::format_description!(
            "[[[day]/[month repr:short]/[year]:[hour]:[minute]:[second] +0100]"
        );
        self.date.format(&format).unwrap()
    }
}

fn get_file_size_bytes(file_path_name: &str) -> std::io::Result<u64> {
    let f = File::open(file_path_name)?;
    let file_size = f.metadata().unwrap().len();
    Ok(file_size)
}

fn get_bytes_from_gigabytes(gigabytes: f32) -> u64 {
    let bytes = gigabytes * 1_000_000_000.0;
    bytes as u64
}

fn get_number_of_logs_to_write(file_size_bytes: u64) -> u64 {
    let min_bytes_of_a_log = 149;
    file_size_bytes / min_bytes_of_a_log + 1
}

struct FileNameGenerator {
    name_suffix: u8,
}

impl FileNameGenerator {
    pub fn new(number_of_files_to_create: u8) -> FileNameGenerator {
        let name_suffix = number_of_files_to_create - 1;
        FileNameGenerator { name_suffix }
    }

    pub fn name(&mut self) -> String {
        let name_prefix = "access.log";
        if self.name_suffix == 0 {
            name_prefix.to_string()
        } else {
            let result = format!("{}.{}", name_prefix, self.name_suffix);
            self.name_suffix -= 1;
            result
        }
    }
}

struct FileCompressor {
    file_path_name: String,
}

impl FileCompressor {
    pub fn new(file_path_name: String) -> FileCompressor {
        FileCompressor { file_path_name }
    }

    pub fn must_compress_the_file(&self) -> bool {
        !self.file_path_name.ends_with(".log") && !self.file_path_name.ends_with(".log.1")
    }

    pub fn compress_file_as_gz(&self) -> Result<(), std::io::Error> {
        let path_name_compressed_file = format!("{}.gz", self.file_path_name);
        let file_gz = File::create(path_name_compressed_file)?;
        let mut enc = GzEncoder::new(file_gz, Compression::default());
        let contents = fs::read_to_string(&self.file_path_name)?;
        enc.write_all(contents.as_bytes())?;
        enc.finish()?;
        Ok(())
    }

    pub fn remove_original_file(&self) -> Result<(), std::io::Error> {
        fs::remove_file(&self.file_path_name)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_add_one_second_twice() {
        let mut date = Date::new(datetime!(2019 - 11 - 26 18:30:59));
        date.add_one_second();
        assert_eq!(datetime!(2019 - 11 - 26 18:31:00), date.date);
        date.add_one_second();
        assert_eq!(datetime!(2019 - 11 - 26 18:31:01), date.date);
    }

    #[test]
    fn date_set_next_day_twice() {
        let mut date = Date::new(datetime!(2019 - 11 - 26 18:30:00));
        date.set_next_day();

        assert_eq!(datetime!(2019 - 11 - 27 00:00:00), date.date);
        date.set_next_day();
        assert_eq!(datetime!(2019 - 11 - 28 00:00:00), date.date);
    }

    #[test]
    fn date_add_one_second_and_set_next_day_update_values_correctly() {
        let mut date = Date::new(datetime!(2019 - 11 - 26 18:30:20));

        date.add_one_second();
        date.set_next_day();
        assert_eq!(datetime!(2019 - 11 - 27 00:00:00), date.date);
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

    #[test]
    fn file_name_generator_creates_all_names_correctly() {
        let number_of_files_to_create = 3;
        let mut file_name_generator = FileNameGenerator::new(number_of_files_to_create);
        assert_eq!("access.log.2", file_name_generator.name());
        assert_eq!("access.log.1", file_name_generator.name());
        assert_eq!("access.log", file_name_generator.name());
    }

    #[test]
    fn file_name_compressor_must_compress_the_file() {
        assert!(!FileCompressor::new("/tmp/access.log".to_string()).must_compress_the_file());
        assert!(!FileCompressor::new("/tmp/access.log.1".to_string()).must_compress_the_file());
        assert!(FileCompressor::new("/tmp/access.log.2".to_string()).must_compress_the_file());
        assert!(FileCompressor::new("/tmp/access.log.10".to_string()).must_compress_the_file());
        assert!(FileCompressor::new("/tmp/access.log.11".to_string()).must_compress_the_file());
    }
}
