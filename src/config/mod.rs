use std::{env::VarError, sync::Arc};

use anyhow::Result;

pub mod crypt;
pub mod database;
pub mod http;
pub mod jwt;
pub mod logging;
pub mod twitch;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub http: Arc<http::Http>,
    pub logging: Arc<logging::Logging>,
    pub database: Arc<database::Database>,
    pub twitch: Arc<twitch::Twitch>,
    pub jwt: Arc<jwt::Jwt>,
    pub crypt: Arc<crypt::Crypt>,
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
            http: Arc::new(http::Http::new()),
            logging: Arc::new(logging::Logging::new()),
            database: Arc::new(database::Database::new()),
            twitch: Arc::new(twitch::Twitch::new()),
            jwt: Arc::new(jwt::Jwt::new()),
            crypt: Arc::new(crypt::Crypt::new()),
        })
    }
}

trait OrDefault {
    fn or_default(&self, default: String) -> String;
}

impl OrDefault for Result<String, VarError> {
    fn or_default(&self, default: String) -> String {
        match self {
            Ok(v) => v.to_string(),
            Err(_) => default,
        }
    }
}
