use std::env;

use types::error::AppResult;

#[derive(Debug, PartialEq, Clone)]
pub struct LoggingConfig {
    level: String,
}

impl LoggingConfig {
    pub fn load() -> AppResult<Self> {
        Ok(LoggingConfig {
            level: env::var("LOGGING_LEVEL").unwrap_or("INFO".to_string()),
        })
    }

    pub fn level(&self) -> &str {
        return &self.level;
    }
}