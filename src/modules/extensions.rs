// extension.rs
// A module for handling file extension.

use std::collections::HashMap;

pub fn recongnize(extension: &str) -> (&'static str, &'static str) {
    // Check if client request for a folder.
    if extension == "" {
        return ("FOLDER", "text/html");
    }

    let map: HashMap<&str, (&str, &str)> = HashMap::from([
        // Text.
        (".css", ("TXT", "text/css")),
        (".csv", ("TXT", "text/csv")),
        (".htm", ("TXT", "text/html")),
        (".html", ("TXT", "text/html")),
        (".js", ("TXT", "text/javascript")),
        (".mjs", ("TXT", "text/javascript")),
        (".txt", ("TXT", "text/plain")),

        // Application.
        (".7z", ("APP", "application/x-7z-compressed")),
        (".abw", ("APP", "application/x-abiword")),
        (".arc", ("APP", "application/x-abiword")),
        (".azw", ("APP", "application/vnd.amazon.ebook")),
        (".bin", ("APP", "application/octet-stream")),
        (".bz", ("APP", "application/x-bzip")),
        (".bz2", ("APP", "application/x-bzip2")),
        (".csh", ("APP", "application/x-csh")),
        (".doc", ("APP", "application/msword")),
        (".docx", ("APP", "application/vnd.openxmlformats-officedocument.wordprocessingml.document")),
        (".eot", ("APP", "application/vnd.ms-fontobject")),
        (".epub", ("APP", "application/epub+zip")),
        (".gz", ("APP", "application/gzip")),
        (".json", ("APP", "application/json")),
        (".jsonld", ("APP", "application/ld+json")),
        (".mpkg", ("APP", "application/vnd.apple.installer+xml")),
        (".odp", ("APP", "application/vnd.oasis.opendocument.presentation")),
        (".ods", ("APP", "application/vnd.oasis.opendocument.spreadsheet")),
        (".odt", ("APP", "application/vnd.oasis.opendocument.text")),
        (".ogx", ("APP", "application/ogg")),
        (".pdf", ("APP", "application/pdf")),
        (".php", ("APP", "application/x-httpd-php")),
        (".ppt", ("APP", "application/vnd.ms-powerpoint")),
        (".pptx", ("APP", "application/vnd.openxmlformats-officedocument.presentationml.presentation")),
        (".rar", ("APP", "application/vnd.rar")),
        (".sh", ("APP", "application/x-sh")),
        (".swf", ("APP", "application/x-shockwave-flash")),
        (".tar", ("APP", "application/x-tar")),
        (".xhtml", ("APP", "application/xhtml+xml")),
        (".xls", ("APP", "application/vnd.ms-excel")),
        (".xlsx", ("APP", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")),
        (".xml", ("APP", "application/xml")),
        (".xul", ("APP", "application/vnd.mozilla.xul+xml")),
        (".zip", ("APP", "applicationzip")),

        // Image.
        (".bmp", ("IMG", "image/bmp")),
        (".gif", ("IMG", "image/gif")),
        (".ico", ("IMG", "image/x-icon")),
        (".jpeg", ("IMG", "image/jpeg")),
        (".jpg", ("IMG", "imagejpeg")),
        (".png", ("IMG", "image/png")),
        (".svg", ("IMG", "image/svg+xml")),
        (".tif", ("IMG", "image/tiff")),
        (".tiff", ("IMG", "image/tiff")),
        (".webp", ("IMG", "image/webp")),

        // Vedio.
        (".3g2", ("VID", "video/3gpp2")),
        (".3gp", ("VID", "video/3gpp")),
        (".avi", ("VID", "video/x-msvideo")),
        (".mkv", ("VID", "video/x-matroska")),
        (".mov", ("VID", "video/quicktime")),
        (".mp4", ("VID", "video/mp4")),
        (".mpeg", ("VID", "video/mpeg")),
        (".ogv", ("VID", "video/ogg")),
        (".ts", ("VID", "video/mp2t")),
        (".webm", ("VID", "video/webm")),

        // Audio.
        (".aac", ("AUD", "audio/aac")),
        (".flac", ("AUD", "audio/flac")),
        (".mid", ("AUD", "audio/midi")),
        (".midi", ("AUD", "audio/x-midi")),
        (".mp3", ("AUD", "audio/mpeg")),
        (".oga", ("AUD", "audio/ogg")),
        (".opus", ("AUD", "audio/opus")),
        (".wav", ("AUD", "audio/wav")),
        (".weba", ("AUD", "audio/weba")),

        // Font.
        (".ttf", ("FNT", "font/ttf")),
        (".otf", ("FNT", "font/otf")),
        (".woff", ("FNT", "font/woff")),
        (".woff2", ("FNT", "font/woff2")),
    ]);

    // Check if file extension is supported.
    if map.contains_key(extension) {
        return *map.get(extension).unwrap();
    }

    // By defualt, Henver-R will return an 404 error page.
    // So the content type is set to "text/html".
    return ("UNKNOWN", "text/html");

    // RFC 2046
    // The "octet-stream" subtype is used to indicate that a body contains arbitrary binary data.
    // return ("UNKNOWN", "application/octet-stream");
}