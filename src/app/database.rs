use config::{ConfigError, Confif as ConfigLib,File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DataBaseConfig {
    pub user: String,
    pub host: String,
    pub dbname:String,
    pub port: u32;
    pub max_attempts:u32;
}
