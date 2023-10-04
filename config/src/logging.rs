use std::env;

use tracing::Level;

use types::error::AppResult;

#[derive(Debug, PartialEq, Clone)]
pub struct LoggingConfig {
    level: Level,
}

impl LoggingConfig {
    pub fn load() -> AppResult<Self> {
        Ok(LoggingConfig {
            level: match env::var("LOGGING_LEVEL").unwrap_or("INFO".to_string()).to_lowercase().as_str() {
                "debug" => Level::DEBUG,
                "error" => Level::ERROR,
                "warn" => Level::WARN,
                "trace" => Level::TRACE,
                _ => Level::INFO
            },
        })
    }

    pub fn level(&self) -> &Level {
        return &self.level;
    }
}