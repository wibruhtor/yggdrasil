use anyhow::Result;
use std::env::{self, VarError};
use tracing::Level;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub http: Http,
    pub logging: Logging,
}

impl Config {
    pub fn new() -> Result<Self> {
        dotenvy::dotenv()?;

        Ok(Config {
            http: Http::new(),
            logging: Logging::new(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Http {
    pub host: String,
    pub port: String,
}

impl Http {
    fn new() -> Self {
        Http {
            host: env::var("HTTP_HOST").or_default("0.0.0.0".to_owned()),
            port: env::var("HTTP_PORT").or_default("3000".to_owned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Logging {
    pub level: tracing::Level,
}

impl Logging {
    fn new() -> Self {
        Logging {
            level: match env::var("LOGGING_LEVEL")
                .or_default("INFO".to_owned())
                .as_str()
            {
                "ERROR" => Level::ERROR,
                "WARN" => Level::WARN,
                "DEBUG" => Level::DEBUG,
                "TRACE" => Level::TRACE,
                _ => Level::INFO,
            },
        }
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
