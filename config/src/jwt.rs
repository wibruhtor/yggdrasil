use std::env;

use types::error::AppResult;

#[derive(Debug, PartialEq, Clone)]
pub struct JwtConfig {
    secret: String,
}

impl JwtConfig {
    pub fn load() -> AppResult<Self> {
        Ok(JwtConfig {
            secret: env::var("JWT_SECRET").expect("fail get JWT_SECRET"),
        })
    }

    pub fn secret(&self) -> &str {
        return &self.secret;
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use fake::{Dummy, Fake, Faker};

    use crate::JwtConfig;

    #[derive(Debug, Dummy)]
    #[allow(dead_code)]
    struct TestData {
        secret: String,
    }

    #[test]
    fn load() {
        for _ in 1..100 {
            let data = Faker.fake::<TestData>();
            env::set_var("JWT_SECRET", &data.secret);

            let config = JwtConfig::load();
            assert!(config.is_ok());
            let config = config.unwrap();
            assert_eq!(config.secret, data.secret);
        }
    }
}
