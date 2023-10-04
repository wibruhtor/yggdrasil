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