use types::error::AppResult;

use crate::crypt::CryptConfig;
use crate::database::DatabaseConfig;
use crate::http::HttpConfig;
use crate::jwt::JwtConfig;
use crate::logging::LoggingConfig;
use crate::twitch::TwitchConfig;

mod logging;
mod jwt;
mod crypt;
mod twitch;
mod database;
mod http;

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    logging_config: LoggingConfig,
    jwt_config: JwtConfig,
    crypt_config: CryptConfig,
    twitch_config: TwitchConfig,
    database_config: DatabaseConfig,
    http_config: HttpConfig,
}

impl Config {
    pub fn load() -> AppResult<Self> {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(e) => {
                // TODO log
            }
        };

        Ok(Config {
            logging_config: LoggingConfig::load()?,
            jwt_config: JwtConfig::load()?,
            crypt_config: CryptConfig::load()?,
            twitch_config: TwitchConfig::load()?,
            database_config: DatabaseConfig::load()?,
            http_config: HttpConfig::load()?,
        })
    }

    pub fn logging_config(&self) -> &LoggingConfig {
        return &self.logging_config;
    }

    pub fn jwt_config(&self) -> &JwtConfig {
        return &self.jwt_config;
    }

    pub fn crypt_config(&self) -> &CryptConfig {
        return &self.crypt_config;
    }

    pub fn twitch_config(&self) -> &TwitchConfig {
        return &self.twitch_config;
    }

    pub fn database_config(&self) -> &DatabaseConfig {
        return &self.database_config;
    }

    pub fn http_config(&self) -> &HttpConfig {
        return &self.http_config;
    }
}