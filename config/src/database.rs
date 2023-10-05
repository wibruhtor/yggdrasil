use std::env;

use types::error::AppResult;

#[derive(Debug, PartialEq, Clone)]
pub struct DatabaseConfig {
    postgres_url: String,
}

impl DatabaseConfig {
    pub fn load() -> AppResult<Self> {
        Ok(DatabaseConfig {
            postgres_url: env::var("DATABASE_URL").expect("fail get DATABASE_URL"),
        })
    }

    pub fn postgres_url(&self) -> &str {
        return &self.postgres_url;
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use fake::{Dummy, Fake, Faker};

    use crate::DatabaseConfig;

    #[derive(Debug, Dummy)]
    #[allow(dead_code)]
    struct TestData {
        postgres_url: String,
    }

    #[test]
    fn load() {
        for _ in 1..100 {
            let data = Faker.fake::<TestData>();
            env::set_var("DATABASE_URL", &data.postgres_url);

            let config = DatabaseConfig::load();
            assert!(config.is_ok());
            let config = config.unwrap();
            assert_eq!(config.postgres_url, data.postgres_url);
        }
    }
}