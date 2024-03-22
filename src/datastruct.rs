use serde_derive::Deserialize;

// Config.
#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: Server,
    pub system: System,
    pub security: Security,
}

impl Config {
    pub fn new() -> Config {
        Config {
            server: Server {
                host: String::new(),
                port: 0,
                default_page: String::new(),
                e400_page: String::new(),
                e403_page: String::new(),
                e404_page: String::new(),
                e409_page: String::new(),
                e414_page: String::new(),
                e500_page: String::new(),
                e502_page: String::new(),
                e503_page: String::new(),
                append_ser_name: String::new(),
            },
            system: System {
                encoding: String::new(),
                logging: String::new(),
                log_file_path: String::new(),
            },
            security: Security {
                allowed_methods: String::new(),
                dir_trav_filter: String::new(),
                clear_url_penc: String::new(),
                url_only_alnum: String::new(),
                max_url_len: 0,
                ban_ip_addr: true,
            },
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub default_page: String,
    pub e400_page: String,
    pub e403_page: String,
    pub e404_page: String,
    pub e409_page: String,
    pub e414_page: String,
    pub e500_page: String,
    pub e502_page: String,
    pub e503_page: String,
    pub append_ser_name: String,
}

#[derive(Clone, Deserialize)]
pub struct System {
    pub encoding: String,
    pub logging: String,
    pub log_file_path: String,
}

#[derive(Clone, Deserialize)]
pub struct Security {
    pub allowed_methods: String,
    pub dir_trav_filter: String,
    pub clear_url_penc: String,
    pub url_only_alnum: String,
    pub max_url_len: i32,
    pub ban_ip_addr: bool,
}

// Request parameter.
#[derive(Deserialize)]
pub struct RequestParam {
    pub method: String,
    pub file: String,
    pub file_type: String,
    pub file_extension: String,
    pub content_type: String,
    pub query: String,
    pub client_ip: String,
    pub user_agent: String,
    pub http_code: i16,
}

impl RequestParam {
    pub fn new() -> RequestParam {
        RequestParam {
            method: String::new(),
            file: String::new(),
            file_type: String::new(),
            file_extension: String::new(),
            content_type: String::new(),
            query: String::new(),
            client_ip: String::new(),
            user_agent: String::new(),
            http_code: 0,
        }
    }
}