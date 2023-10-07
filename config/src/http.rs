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
            allow_origin: env::var("HTTP_ALLOW_ORIGIN")
                .unwrap_or("http://localhost:3000".to_string()),
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

#[cfg(test)]
mod tests {
    use std::env;

    use fake::{Dummy, Fake, Faker};

    use crate::HttpConfig;

    #[derive(Debug, Dummy)]
    #[allow(dead_code)]
    struct TestData {
        host: String,
        port: String,
        allow_origin: String,
    }

    #[test]
    fn load() {
        for _ in 1..100 {
            let data = Faker.fake::<TestData>();
            env::set_var("HTTP_HOST", &data.host);
            env::set_var("HTTP_PORT", &data.port);
            env::set_var("HTTP_ALLOW_ORIGIN", &data.allow_origin);

            let config = HttpConfig::load();
            assert!(config.is_ok());
            let config = config.unwrap();
            assert_eq!(config.host, data.host);
            assert_eq!(config.port, data.port);
            assert_eq!(config.allow_origin, data.allow_origin);
        }
    }
}
