use std::env;

use types::error::AppResult;

#[derive(Debug, PartialEq, Clone)]
pub struct TwitchConfig {
    callback_url: String,
    client_id: String,
    client_secret: String,
}

impl TwitchConfig {
    pub fn load() -> AppResult<Self> {
        Ok(TwitchConfig {
            callback_url: env::var("TWITCH_CALLBACK_URL").expect("fail get TWITCH_CALLBACK_URL"),
            client_id: env::var("TWITCH_CLIENT_ID").expect("fail get TWITCH_CLIENT_ID"),
            client_secret: env::var("TWITCH_CLIENT_SECRET").expect("fail get TWITCH_CLIENT_SECRET"),
        })
    }

    pub fn callback_url(&self) -> &str {
        return &self.callback_url;
    }

    pub fn client_id(&self) -> &str {
        return &self.client_id;
    }

    pub fn client_secret(&self) -> &str {
        return &self.client_secret;
    }
}