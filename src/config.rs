use crate::logger;
use crate::datastruct::Config;

use toml;
use std::fs;
use std::thread;
use std::time::Duration;

pub fn read() -> Config {
    // Defualt variables.
    let filename = "src/conf/config.toml";
    let mut content: String = String::new();
    let mut config: Config = Config::new();

    // thread::sleep(Duration::from_millis(1000));

    // Open config file and read contents.
    match fs::read_to_string(filename) {
        Ok(file_content) => content = file_content,
        Err(_) => logger::entry(1, format!("Cannot open config file: {}", filename), true, true, true),
    };

    // Parse contents.
    match toml::from_str(&content) {
        Ok(data) => config = data,
        Err(error) => logger::entry(1, error.to_string(), true, true, true),
    };

    return config;
}