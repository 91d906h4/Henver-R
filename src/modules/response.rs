// response.rs
// A module for generating and sending back the response to clients.

use crate::config;
use crate::datastruct::{Config, RequestParam};

use std::fs;
use std::io::Write;
use std::path::Path;
use std::net::TcpStream;
use chrono::prelude::Local;

// Set static value.
const ROOT: &str = "src/public/";
const VERSION: &str = "1.1.0";

pub fn entry(mut request_param: RequestParam, mut request: TcpStream, config: &Config) {
    // Server name.
    let mut server_name: String = String::new();
    if config.server.append_ser_name {
        server_name = format!("Server: Henver-R {}\r\n", VERSION);
    }

    // Get file contents.
    let content: Vec<u8> = content_reader(&mut request_param);

    // Get now date.
    let date = Local::now().format("%Y/%m/%d %H:%M:%S%.3f");

    // Generate header.
    let header: String = format!(
        "HTTP/1.1 {} {}\r\n\
        {}\
        Date: {}\r\n\
        Content-Type: {}\r\n\r\n",

        request_param.http_code, status_message(request_param.http_code),
        server_name,
        date,
        request_param.content_type,
    );

    // Send header.
    request.write(header.as_bytes()).unwrap();

    // Send Content.
    request.write(&content).unwrap();

    request.flush().unwrap();
}

fn content_reader(request_param: &mut RequestParam) -> Vec<u8> {
    // Get configuration.
    let config: Config = config::read();

    // Defualt page.
    if request_param.file == "/" {
        request_param.file = config.server.default_page;
        request_param.file_type = String::from("TXT");
        request_param.content_type = String::from("text/html");
        request_param.http_code = 200;
    }

    // Check if error.
    if request_param.http_code != 200 {
        // Pass if error has already been detected.
    }
    else if request_param.file_type == "FOLDER" {
        // Pass if client request a folder.
        request_param.http_code = 403;
        request_param.content_type = String::from("text/html");
    }
    else if Path::new(format!("{}{}", ROOT, request_param.file).as_str()).exists() {
        // Check if file exists.
        request_param.http_code = 200;
    }
    else {
        // Otherwise, return 404.
        request_param.http_code = 404;
        request_param.content_type = String::from("text/html");
    }

    let filepath: String = match request_param.http_code {
        // 2XX
        200 => (*request_param.file).to_string(),

        // 4XX
        400 => config.server.e400_page,
        403 => config.server.e403_page,
        404 => config.server.e404_page,
        409 => config.server.e409_page,
        414 => config.server.e414_page,

        // 5XX
        500 => config.server.e500_page,
        502 => config.server.e502_page,
        503 => config.server.e503_page,

        // Else.
        _ => config.server.e404_page,
    };

    if request_param.file_type == "TXT" {
        // Read as string if file is a text file.
        // (Only text file needs to be read as string)
        return fs::read_to_string(format!("{}{}", ROOT, filepath)).unwrap().into();
    }
    else {
        // Read as bytes if file is a media file (image, video, audio, and font)
        // or the file type is application or unknown.
        return fs::read(format!("{}{}", ROOT, filepath)).unwrap();
    }
    
}

fn status_message(code: i16) -> &'static str {
    match code {
        // 2XX
        200 => "OK",

        // 4XX
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        409 => "Too Many Requests",
        414 => "URI Too Long",

        // 5XX
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",

        // Else.
        _ => "Unknown",
    }
}