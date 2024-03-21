#![allow(non_snake_case)]
mod config;
mod logger;
mod layers;
mod modules;
mod datastruct;

use datastruct::Config;
use std::{
    io::Read,
    net::{TcpListener, TcpStream}
};

fn main() {
    // Get configuration.
    let config: Config = config::read();

    // Get address.
    let address: String = format!("{}:{}", config.server.host, config.server.port);

    // Create listener.
    let listener: TcpListener = TcpListener::bind(address.clone()).unwrap();

    // Welcome message.
    println!("Thank you for using Hanver-R HTTP server!");
    println!("Hanver-R is now listening on {}.", address);
    println!("Use CTRL+C to stop the server.");

    // Accept connections from clients.
    for stream in listener.incoming() {
        match stream {
            Ok(request) => {
                std::thread::spawn(move || index(request));
            },
            Err(error) => {
                logger::entry(1, error.to_string(), false, true, true);
            }
        }
    }
}

fn index(mut request: TcpStream) {
    // Client address.
    let client_address: String = request.peer_addr().unwrap().to_string();

    logger::entry(3, format!("Connection from {}.", client_address).to_string(), false, false, true);

    // HTTP request.
    let mut buffer: [u8; 1024] = [0; 1024];
    request.read(&mut buffer).unwrap();

    // Get HTTP request.
    let http_request: String = String::from_utf8_lossy(&buffer).to_string();

    // Enter layers.
    layers::entry(request, &http_request, &client_address);
}