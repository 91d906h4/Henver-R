// extension.rs
// A module for handling file extension.

use std::collections::HashMap;

pub fn recongnize(extension: &str) -> (&'static str, &'static str) {
    // Check if client request for a folder.
    if extension == "" {
        return ("FOLDER", "");
    }

    let map: HashMap<&str, (&str, &str)> = HashMap::from([
        // Text.
        (".css",        ("TXT", "text/css")),
        (".csv",        ("TXT", "text/csv")),
        (".htc",        ("TXT", "text/x-component")),
        (".htm",        ("TXT", "text/html")),
        (".html",       ("TXT", "text/html")),
        (".jad",        ("TXT", "text/vnd.sun.j2me.app-descriptor")),
        (".js",         ("TXT", "text/javascript")),
        (".mjs",        ("TXT", "text/javascript")),
        (".txt",        ("TXT", "text/plain")),

        // Application.
        (".7z",         ("APP", "application/x-7z-compressed")),
        (".abw",        ("APP", "application/x-abiword")),
        (".ai",         ("APP", "application/postscript")),
        (".arc",        ("APP", "application/x-abiword")),
        (".atom",       ("APP", "application/atom+xml")),
        (".azw",        ("APP", "application/vnd.amazon.ebook")),
        (".bin",        ("APP", "application/octet-stream")),
        (".bz",         ("APP", "application/x-bzip")),
        (".bz2",        ("APP", "application/x-bzip2")),
        (".cco",        ("APP", "application/x-cocoa")),
        (".crt",        ("APP", "application/x-x509-ca-cert")),
        (".csh",        ("APP", "application/x-csh")),
        (".doc",        ("APP", "application/msword")),
        (".deb",        ("APP", "application/octet-stream")),
        (".der",        ("APP", "application/x-x509-ca-cert")),
        (".dll",        ("APP", "application/octet-stream")),
        (".dmg",        ("APP", "application/octet-stream")),
        (".docx",       ("APP", "application/vnd.openxmlformats-officedocument.wordprocessingml.document")),
        (".ear",        ("APP", "application/java-archive")),
        (".eot",        ("APP", "application/vnd.ms-fontobject")),
        (".eps",        ("APP", "application/postscript")),
        (".epub",       ("APP", "application/epub+zip")),
        (".exe",        ("APP", "application/octet-stream")),
        (".gz",         ("APP", "application/gzip")),
        (".hqx",        ("APP", "application/mac-binhex40")),
        (".img",        ("APP", "application/octet-stream")),
        (".iso",        ("APP", "application/octet-stream")),
        (".jar",        ("APP", "application/java-archive")),
        (".jardiff",    ("APP", "application/x-java-archive-diff")),
        (".jnlp",       ("APP", "application/x-java-jnlp-file")),
        (".json",       ("APP", "application/json")),
        (".jsonld",     ("APP", "application/ld+json")),
        (".kml",        ("APP", "application/vnd.google-earth.kml+xml")),
        (".kmz",        ("APP", "application/vnd.google-earth.kmz")),
        (".m3u8",       ("APP", "application/vnd.apple.mpegurl")),
        (".mpkg",       ("APP", "application/vnd.apple.installer+xml")),
        (".msi",        ("APP", "application/octet-stream")),
        (".msm",        ("APP", "application/octet-stream")),
        (".msp",        ("APP", "application/octet-stream")),
        (".odg",        ("APP", "application/vnd.oasis.opendocument.graphics")),
        (".odp",        ("APP", "application/vnd.oasis.opendocument.presentation")),
        (".ods",        ("APP", "application/vnd.oasis.opendocument.spreadsheet")),
        (".odt",        ("APP", "application/vnd.oasis.opendocument.text")),
        (".ogx",        ("APP", "application/ogg")),
        (".pdb",        ("APP", "application/x-pilot")),
        (".pdf",        ("APP", "application/pdf")),
        (".epm",        ("APP", "application/x-x509-ca-cert")),
        (".php",        ("APP", "application/x-httpd-php")),
        (".pl",         ("APP", "application/x-perl")),
        (".pm",         ("APP", "application/x-perl")),
        (".ppt",        ("APP", "application/vnd.ms-powerpoint")),
        (".pptx",       ("APP", "application/vnd.openxmlformats-officedocument.presentationml.presentation")),
        (".prc",        ("APP", "application/x-pilot")),
        (".ps",         ("APP", "application/postscript")),
        (".rar",        ("APP", "application/vnd.rar")),
        (".rpm",        ("APP", "application/x-redhat-package-manager")),
        (".rss",        ("APP", "application/rss+xml")),
        (".rtf",        ("APP", "application/rtf")),
        (".run",        ("APP", "application/x-makeself")),
        (".sea",        ("APP", "application/x-sea")),
        (".sh",         ("APP", "application/x-sh")),
        (".sit",        ("APP", "application/x-stuffit")),
        (".swf",        ("APP", "application/x-shockwave-flash")),
        (".tar",        ("APP", "application/x-tar")),
        (".tcl",        ("APP", "application/x-tcl")),
        (".tk",         ("APP", "application/x-tcl")),
        (".war",        ("APP", "application/java-archive")),
        (".wasm",       ("APP", "application/wasm")),
        (".wmlc",       ("APP", "application/vnd.wap.wmlc")),
        (".xhtml",      ("APP", "application/xhtml+xml")),
        (".xls",        ("APP", "application/vnd.ms-excel")),
        (".xlsx",       ("APP", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")),
        (".xml",        ("APP", "application/xml")),
        (".xpi",        ("APP", "application/x-xpinstall")),
        (".xspf",       ("APP", "application/xspf+xml")),
        (".xul",        ("APP", "application/vnd.mozilla.xul+xml")),
        (".zip",        ("APP", "applicationzip")),

        // Image.
        (".bmp",        ("IMG", "image/x-ms-bmp")),
        (".gif",        ("IMG", "image/gif")),
        (".ico",        ("IMG", "image/x-icon")),
        (".jng",        ("IMG", "image/x-jng")),
        (".jpeg",       ("IMG", "image/jpeg")),
        (".jpg",        ("IMG", "imagejpeg")),
        (".png",        ("IMG", "image/png")),
        (".svg",        ("IMG", "image/svg+xml")),
        (".svg2",       ("IMG", "image/svg+xml")),
        (".tif",        ("IMG", "image/tiff")),
        (".tiff",       ("IMG", "image/tiff")),
        (".wbmp",       ("IMG", "image/vnd.wap.wbmp")),
        (".webp",       ("IMG", "image/webp")),

        // Vedio.
        (".3g2",        ("VID", "video/3gpp2")),
        (".3gp",        ("VID", "video/3gpp")),
        (".3gpp",       ("VID", "video/3gpp")),
        (".asf",        ("VID", "video/x-ms-asf")),
        (".asx",        ("VID", "video/x-ms-asf")),
        (".avi",        ("VID", "video/x-msvideo")),
        (".fiv",        ("VID", "video/x-flv")),
        (".m4v",        ("VID", "video/x-m4v")),
        (".mkv",        ("VID", "video/x-matroska")),
        (".mov",        ("VID", "video/quicktime")),
        (".mp4",        ("VID", "video/mp4")),
        (".mpg",        ("VID", "video/mpeg")),
        (".mpeg",       ("VID", "video/mpeg")),
        (".ogv",        ("VID", "video/ogg")),
        (".ts",         ("VID", "video/mp2t")),
        (".webm",       ("VID", "video/webm")),
        (".wmv",        ("VID", "video/x-ms-wmv")),

        // Audio.
        (".aac",        ("AUD", "audio/aac")),
        (".flac",       ("AUD", "audio/flac")),
        (".kar",        ("AUD", "audio/midi")),
        (".mid",        ("AUD", "audio/midi")),
        (".midi",       ("AUD", "audio/x-midi")),
        (".m4a",        ("AUD", "audio/x-m4a")),
        (".mp3",        ("AUD", "audio/mpeg")),
        (".oga",        ("AUD", "audio/ogg")),
        (".ogg",        ("AUD", "audio/ogg")),
        (".opus",       ("AUD", "audio/opus")),
        (".ra",         ("AUD", "audio/x-realaudio")),
        (".wav",        ("AUD", "audio/wav")),
        (".weba",       ("AUD", "audio/weba")),

        // Font.
        (".ttf",        ("FNT", "font/ttf")),
        (".otf",        ("FNT", "font/otf")),
        (".woff",       ("FNT", "font/woff")),
        (".woff2",      ("FNT", "font/woff2")),
    ]);

    // Check if file extension is supported.
    if map.contains_key(extension) {
        return *map.get(extension).unwrap();
    }

    // RFC 2046
    // The "octet-stream" subtype is used to indicate that a body contains arbitrary binary data.
    return ("UNKNOWN", "application/octet-stream");
}