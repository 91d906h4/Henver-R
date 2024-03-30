use crate::logger;
use crate::datastruct::Config;

use toml;
use std::{fs, path::Path};

// Set static value.
const ROOT: &str = "src/public/";

pub fn read() -> Config {
    // Defualt variables.
    let filename = "src/conf/config.toml";
    let mut content: String = String::new();
    let mut config: Config = Config::new();

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

    // Check if error 500 page exists.
    if !Path::new(format!("{}{}", ROOT, config.server.e500_page).as_str()).exists() {
        logger::entry(1, "Error 500 page is misconfigured, please check the path in config file.".to_string(), true, true, true);
    }

    return config;
}