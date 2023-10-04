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