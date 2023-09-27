use std::env;

use super::OrDefault;

#[derive(Debug, Clone, PartialEq)]
pub struct Http {
    pub host: String,
    pub port: String,
    pub allow_origin: String,
}

impl Http {
    pub fn new() -> Self {
        Http {
            host: env::var("HTTP_HOST").or_default("0.0.0.0".to_string()),
            port: env::var("HTTP_PORT").or_default("8000".to_string()),
            allow_origin: env::var("HTTP_ALLOW_ORIGIN")
                .or_default("http://localhost:3000".to_string()),
        }
    }
}
