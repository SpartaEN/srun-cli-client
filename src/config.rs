use crate::srun::client::{SRUNClient, SRUNClientCredentials, SRUNClientOptions};
use json;
use std::fs;
use std::option::Option;

#[derive(Debug)]
pub enum OutputFormat {
    Plain,
    Json,
}

#[derive(Debug)]
pub struct AppConfig {
    pub username: Option<String>,
    pub password: Option<String>,
    pub server: Option<String>,
    pub redirect: bool,
    pub redirect_host: Option<String>,
    pub interface: Option<String>,
    pub output: OutputFormat,
    pub command: Option<String>,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            username: None,
            password: None,
            server: None,
            redirect: false,
            redirect_host: None,
            interface: None,
            output: OutputFormat::Plain,
            command: None,
        }
    }

    pub fn from_file(config_path: &String) -> AppConfig {
        let mut app_config = AppConfig::new();
        let content = fs::read_to_string(config_path);
        match content {
            Ok(s) => {
                let json = json::parse(s.as_str()).unwrap();
                app_config.username = json["username"].as_str().map(String::from);
                app_config.password = json["password"].as_str().map(String::from);
                app_config.server = json["server"].as_str().map(String::from);
                app_config.redirect = json["redirect"].as_bool().unwrap_or(false);
                app_config.redirect_host = json["redirect_host"].as_str().map(String::from);
                app_config.interface = json["interface"].as_str().map(String::from);
                app_config.output = match json["output"].as_str() {
                    Some("json") => OutputFormat::Json,
                    _ => OutputFormat::Plain,
                };
                app_config
            }
            Err(_) => {
                println!("Failed to read config file");
                std::process::exit(1);
            }
        }
    }
}

impl SRUNClient {
    pub fn from_app_config(config: &AppConfig) -> SRUNClient {
        let mut client = SRUNClient {
            server: config.server.clone().unwrap(),
            credentials: None,
            options: SRUNClientOptions {
                redirect_host: config
                    .redirect_host
                    .clone()
                    .unwrap_or(String::from("http://www.google.cn/generate_204")),
                interface: config.interface.clone(),
            },
        };
        match config.username.clone() {
            Some(username) => match config.password.clone() {
                Some(password) => {
                    client.credentials = Some(SRUNClientCredentials {
                        username: username,
                        password: password,
                    })
                }
                None => {}
            },
            None => {}
        }
        client
    }
}
