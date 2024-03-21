// queryparse.rs
// A module for parsing query string.

use crate::modules::extensions;
use crate::datastruct::RequestParam;

pub fn entry(mut request_param: RequestParam) -> RequestParam {
    (request_param.file, request_param.file_extension, request_param.query) = parser(&request_param.query);

    // To recognize the file type by extension and set content type.
    let (file_type, content_type) = extensions::recongnize(&request_param.file_extension);
    (request_param.file_type, request_param.content_type) = (file_type.to_string(), content_type.to_string());

    return request_param;
}

fn parser(query: &str) -> (String, String, String) {
    let temp: usize = query.find('?').unwrap_or(query.len());

    // Get query string.
    let clean_query: String = query[temp..query.len()].to_string();

    // Get filename.
    let file: String = query[0..temp].to_string();

    // Get file extension.
    let temp: &str = file.split('/').last().unwrap();
    let mut file_extension: String = String::new();

    if temp.contains('.') {
        file_extension = temp[temp.rfind('.').unwrap_or(0)..temp.len()].to_string();
    }

    return (file, file_extension, clean_query);
}