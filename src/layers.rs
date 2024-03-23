#![allow(unused_imports)]
use crate::datastruct::{
    Config,
    RequestParam
};
use crate::modules::{
    request,
    security,
    queryparse,
    anything,
    response,
};

use std::net::TcpStream;

/// ## Layers
/// 
/// Use the layers to handle requests. You can modify and add layers here.<br />
/// (Remember to modify the `mod.rs` file)
/// 
/// For example:
/// 
/// ```
/// // Request handle.
/// let layer1 = request::entry(...);
/// // Security check.
/// let layer2 = security::entry(...);
/// // Query parse.
/// let layer3 = queryparse::entry(...);
/// ...
/// response::entry(...);
/// ```
pub fn entry(request: TcpStream, http_request: &str, client_address: &str, config: &Config) {
    // Handle request.
    let request_param: RequestParam = request::entry(&client_address, &http_request);

    // Security check.
    let request_param: RequestParam = security::entry(&client_address, request_param, request.try_clone().unwrap(), &config);

    // Query parse.
    let request_param: RequestParam = queryparse::entry(request_param);

    // Test layser.
    // Replace this layer with yours.
    // let request_param: RequestParam = anything::entry(request_param);

    // Response.
    response::entry(request_param, request, &config);
}