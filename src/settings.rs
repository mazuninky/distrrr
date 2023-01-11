use config::{Config, ConfigError, Environment, File};
use std::env;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostgresConfiguration {
    pub connect_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub port: u16,
    pub postgres: PostgresConfiguration,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config.yaml").required(false))
            .add_source(Environment::with_prefix("distrrr"))
            .build()?;

        s.try_deserialize()
    }
}
