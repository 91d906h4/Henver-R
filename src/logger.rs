use std::process::exit;
use chrono::prelude::Local;

/// ## Error logger
/// 
/// Use `error::entry()` to log error messages.
///
/// ### Error Code:
///     1 - Error
///     2 - Warning
///     3 - Info
///     _ - Unxepected Error
/// 
pub fn entry(code: i8, mut message: String, exits: bool, shows: bool, logs: bool) {
    // Handle error code.
    let header = match code {
        1 => "[Error]",
        2 => "[Warning]",
        3 => "[Info]",
        _ => "[Unxepected Error]",
    };

    // Get now time.
    let time = Local::now().format("%Y/%m/%d %H:%M:%S%.3f");

    // Check if error message is provided.
    if message.is_empty() {
        message = String::from("No error message provided.");
    }

    // Generate message.
    message = format!("{} {} | {}", header, time, message);

    // Show message on console.
    if shows {
        println!("{}", message);
    }

    // Log message.
    if logs { }

    // Stop server.
    if exits {
        exit(1);
    }
}