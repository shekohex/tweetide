use std::net::SocketAddr;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub debug: bool,
    pub server_addr: SocketAddr,
    pub database: Database,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        let env = Environment::new().prefix("APP");
        s.merge(File::with_name("config"))?;
        s.merge(env)?;
        s.try_into()
    }
}
