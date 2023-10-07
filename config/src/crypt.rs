use std::env;

use types::error::AppResult;

#[derive(Debug, PartialEq, Clone)]
pub struct CryptConfig {
    secret: String,
}

impl CryptConfig {
    pub fn load() -> AppResult<Self> {
        Ok(CryptConfig {
            secret: env::var("CRYPT_SECRET").expect("fail get CRYPT_SECRET"),
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

    use crate::CryptConfig;

    #[derive(Debug, Dummy)]
    #[allow(dead_code)]
    struct TestData {
        secret: String,
    }

    #[test]
    fn load() {
        for _ in 1..100 {
            let data = Faker.fake::<TestData>();
            env::set_var("CRYPT_SECRET", &data.secret);

            let config = CryptConfig::load();
            assert!(config.is_ok());
            let config = config.unwrap();
            assert_eq!(config.secret, data.secret);
        }
    }
}
