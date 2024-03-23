// security.rs
// A module for security check.

use crate::logger;
use crate::datastruct::{Config, RequestParam};

use std::fs;
use urlencoding::decode;
use std::net::TcpStream;
use std::collections::HashSet;

pub fn entry(client_address: &str, mut request_param: RequestParam, request: TcpStream, config: &Config) -> RequestParam {
    // Check if client is banned.
    // if config.security.ban_ip_addr {
    //     let client_address = &client_address[0..client_address.find(':').unwrap_or(client_address.len())];

    //     for ip in fs::read_to_string("src/conf/banned_ip.txt").unwrap().lines() {
    //         if client_address == ip {
    //             // Shutdown connection.
    //             request.shutdown(std::net::Shutdown::Both).unwrap();
        
    //             logger::entry(2, "Got connection from banned IP address.".to_string(), false, true, true);
    //         }
    //     }
    // }

    // Check HTTP methods.
    if config.security.allowed_methods != "all" &&
       !config.security.allowed_methods.contains(request_param.method.to_lowercase().as_str()) {
        // Shutdown connection.
        request.shutdown(std::net::Shutdown::Both).unwrap();

        logger::entry(2, "Got illegal HTTP method.".to_string(), false, true, true);
    }

    // Check request length.
    if config.security.max_url_len == -1 {
        // Pass if max length is setted to -1.
    }
    else if request_param.query.len() > config.security.max_url_len as usize {
        // Shutdown connection.
        // request.shutdown(std::net::Shutdown::Both).unwrap();

        request_param.http_code = 414;

        logger::entry(2, "Got too long request.".to_string(), false, true, true);
    }

    // Clear percenrage encoding.
    if config.security.clear_url_penc == "enable" {
        let mut temp: String = request_param.query;

        // Remove all percentage encoding chars.
        while temp.contains('%') {
            let check: &str = &temp.clone();
            temp = decode(&temp).expect("utf-8").to_string();

            // Check if the string is not changed.
            if temp == check { break; }
        }

        request_param.query = temp;
    }

    // Clear illegal chars.
    if config.security.url_only_alnum == "enable" {
        // Chars map.
        let map: HashSet<char> = HashSet::from([
            // 0 - 9
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',

            // a - z
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
            'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
            'u', 'v', 'w', 'x', 'y', 'z',

            // A - Z
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
            'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
            'U', 'V', 'W', 'X', 'Y', 'Z',

            // Special chars.
            '_', '?', '&', '/', '=', '-', '.', ':', '+', '(',
            ')', '#',
        ]);

        let mut temp: String = String::new();

        // Filter illegal chars.
        for i in request_param.query.chars() {
            if map.contains(&i) {
                temp += &i.to_string();
            }
        }

        request_param.query = temp;
    }

    // Filter directory traversal.
    if config.security.dir_trav_filter == "enable" {
        let mut temp: String = request_param.query;

        // Replace ".." and "//".
        while temp.contains("..") || temp.contains("//") {
            temp = temp.replace("..", "");
            temp = temp.replace("//", "");
        }

        request_param.query = temp;
    }

    return request_param;
}