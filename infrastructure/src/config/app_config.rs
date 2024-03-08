// infrastructure\src\config\app_config.rs

use dirs::home_dir;
use serde::{Serialize, Deserialize};
use std::fs;
use toml;

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn load_or_init() -> Result<Self, std::io::Error> {
        let home_dir = home_dir().expect("Failed to find home directory");
        let config_dir = home_dir.join(".config/sebden");
        let config_path = config_dir.join("config.toml");

        if !config_path.exists() {
            let default_config = AppConfig {
                server: ServerConfig { port: 7041 },
                database: DatabaseConfig { url: "postgres://user:password@localhost/sebden".to_string() },
            };
            fs::create_dir_all(&config_dir)?;
            let config_str = toml::to_string(&default_config).expect("Failed to serialize config");
            fs::write(&config_path, config_str)?;
            Ok(default_config)
        } else {
            let config_str = fs::read_to_string(&config_path)?;
            let config: AppConfig = toml::from_str(&config_str).expect("Failed to deserialize config");
            Ok(config)
        }
    }
}