use serde::Deserialize;
use tokio::fs;
use toml;
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub retry: u16,
}

#[derive(Debug, Deserialize)]
pub struct HTTPServerConfig{
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Option<DatabaseConfig>,
    pub http_server: Option<HTTPServerConfig>,
}

impl Config{
    pub async fn new() -> Self{
        let config_path = if cfg!(debug_assertions) {
            println!("Debug mode");
            "config/dev.toml"
        } else {
            println!("Prod mode");
            "config/prod.toml"
        };

        let content = fs::read_to_string(config_path).await.unwrap_or_else(|err| {
            println!("Error reading config file: {:?}", err);
            String::new()
        });

        toml::from_str(&content).unwrap_or_else(|err| {
            println!("Error parsing config file: {:?}", err);
            Config {
                database: None,
                http_server: None,
            }
        })
    }
}