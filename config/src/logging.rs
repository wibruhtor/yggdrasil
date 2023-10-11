use std::env;

use tracing::Level;

use types::error::AppResult;

#[derive(Debug, PartialEq, Clone)]
pub struct LoggingConfig {
    level: Level,
}

impl LoggingConfig {
    pub fn load() -> AppResult<Self> {
        let config = LoggingConfig {
            level: match env::var("LOGGING_LEVEL")
                .unwrap_or("INFO".to_string())
                .to_lowercase()
                .as_str()
            {
                "debug" => Level::DEBUG,
                "error" => Level::ERROR,
                "warn" => Level::WARN,
                "trace" => Level::TRACE,
                _ => Level::INFO,
            },
        };

        env::set_var("RUST_LOG", config.level.as_str().to_lowercase());

        Ok(config)
    }

    pub fn level(&self) -> &Level {
        return &self.level;
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use fake::{faker::number::en::Digit, Dummy, Fake, Faker};
    use tracing::Level;

    use crate::LoggingConfig;

    #[derive(Debug, Dummy)]
    #[allow(dead_code)]
    struct TestData {
        #[dummy(faker = "Digit()")]
        level: String,
    }

    impl TestData {
        fn level(&self) -> String {
            let level = self.level.parse::<i32>();
            assert!(level.is_ok());
            let level = level.unwrap();
            if (0..2).contains(&level) {
                "deBuG".to_string()
            } else if (2..4).contains(&level) {
                "ErroR".to_string()
            } else if (4..6).contains(&level) {
                "WARN".to_string()
            } else if (6..8).contains(&level) {
                "trace".to_string()
            } else {
                (8..20).fake::<String>()
            }
        }
        fn valid_level(&self) -> Level {
            let level = self.level.parse::<i32>();
            assert!(level.is_ok());
            let level = level.unwrap();
            if (0..2).contains(&level) {
                Level::DEBUG
            } else if (2..4).contains(&level) {
                Level::ERROR
            } else if (4..6).contains(&level) {
                Level::WARN
            } else if (6..8).contains(&level) {
                Level::TRACE
            } else {
                Level::INFO
            }
        }
    }

    #[test]
    fn load() {
        for _ in 1..100 {
            let data = Faker.fake::<TestData>();
            env::set_var("LOGGING_LEVEL", data.level());

            let config = LoggingConfig::load();
            assert!(config.is_ok());
            let config = config.unwrap();
            assert_eq!(config.level, data.valid_level());
        }
    }
}
