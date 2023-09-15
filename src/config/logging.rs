use std::env;

use tracing::Level;

use super::OrDefault;

#[derive(Debug, Clone, PartialEq)]
pub struct Logging {
    pub level: tracing::Level,
}

impl Logging {
    pub fn new() -> Self {
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
