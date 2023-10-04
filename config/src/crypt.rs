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