use std::env;

use super::OrDefault;

#[derive(Debug, Clone, PartialEq)]
pub struct Http {
    pub host: String,
    pub port: String,
}

impl Http {
    pub fn new() -> Self {
        Http {
            host: env::var("HTTP_HOST").or_default("0.0.0.0".to_string()),
            port: env::var("HTTP_PORT").or_default("3000".to_string()),
        }
    }
}
