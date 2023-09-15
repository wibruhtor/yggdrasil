use anyhow::Result;
use std::env::VarError;

mod database;
mod http;
mod logging;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub http: http::Http,
    pub logging: logging::Logging,
    pub database: database::Database,
}

impl Config {
    pub fn new() -> Result<Self> {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(e) => {
                tracing::warn!("fail load dotenv: {}", e)
            }
        };

        Ok(Config {
            http: http::Http::new(),
            logging: logging::Logging::new(),
            database: database::Database::new(),
        })
    }
}

trait OrDefault {
    fn or_default(&self, default: String) -> String;
}

impl OrDefault for Result<String, VarError> {
    fn or_default(&self, default: String) -> String {
        match self {
            Ok(v) => v.to_owned(),
            Err(_) => default,
        }
    }
}
