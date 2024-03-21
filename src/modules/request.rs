// request.rs
// A module to parse the requests receive from clients.

use crate::datastruct::RequestParam;

pub fn entry(client_address: &str, http_request: &str) -> RequestParam {
    // Default values.
    let mut request_param: RequestParam = RequestParam::new();

    // Set defualt values.
    request_param.client_ip = client_address.to_string();
    request_param.http_code = 200;

    // Parse HTTP header.
    for line in http_request.split('\n') {
        // Get HTTP method.
        if line.starts_with("GET") || line.starts_with("POST") ||
           line.starts_with("HEAD") || line.starts_with("PUT") {
            let temp: Vec<&str> = line.split(' ').collect();
            request_param.method = temp[0].to_string();
            request_param.query = temp[1].to_string();
        }

        // Get user agent.
        else if line.starts_with("User-Agent") {
            let (_, temp) = line.split_once(' ').unwrap();
            request_param.user_agent = temp.trim().to_string();

            // In normal case, HTTP method should be in front of User-Agent.
            // So we can implement early stop if user agent is found to achieve
            // higher performance.
            break;
        }
    }
    
    return request_param;
}