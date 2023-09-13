use anyhow::Result;
use std::env::{self, VarError};

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub http: Http,
}

impl Config {
    pub fn new() -> Result<Self> {
        dotenvy::dotenv()?;

        Ok(Config { http: Http::new() })
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
            host: env::var("HOST").or_default("0.0.0.0".to_owned()),
            port: env::var("PORT").or_default("3000".to_owned()),
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
