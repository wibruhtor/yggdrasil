use std::env;

use types::error::AppResult;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpConfig {
    host: String,
    port: String,
    allow_origin: String,
}

impl HttpConfig {
    pub fn load() -> AppResult<Self> {
        Ok(HttpConfig {
            host: env::var("HTTP_HOST").unwrap_or("0.0.0.0".to_string()),
            port: env::var("HTTP_PORT").unwrap_or("8000".to_string()),
            allow_origin: env::var("HTTP_ALLOW_ORIGIN").unwrap_or("http://localhost:3000".to_string()),
        })
    }

    pub fn host(&self) -> &str {
        return &self.host;
    }

    pub fn port(&self) -> &str {
        return &self.port;
    }

    pub fn allow_origin(&self) -> &str {
        return &self.allow_origin;
    }
}