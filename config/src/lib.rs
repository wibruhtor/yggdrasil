pub use crypt::*;
pub use database::*;
pub use http::*;
pub use jwt::*;
pub use logging::*;
pub use twitch::*;
use types::error::AppResult;

mod crypt;
mod database;
mod http;
mod jwt;
mod logging;
mod twitch;

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
        let span = tracing::debug_span!("load config");
        let _span = span.enter();

        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(e) => {
                tracing::warn!("fail load dotenv: {}", e)
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
