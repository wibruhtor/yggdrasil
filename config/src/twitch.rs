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

#[cfg(test)]
mod tests {
    use std::env;

    use fake::{Dummy, Fake, Faker};

    use crate::TwitchConfig;

    #[derive(Debug, Dummy)]
    #[allow(dead_code)]
    struct TestData {
        callback_url: String,
        client_id: String,
        client_secret: String,
    }

    #[test]
    fn load() {
        for _ in 1..100 {
            let data = Faker.fake::<TestData>();
            env::set_var("TWITCH_CALLBACK_URL", &data.callback_url);
            env::set_var("TWITCH_CLIENT_ID", &data.client_id);
            env::set_var("TWITCH_CLIENT_SECRET", &data.client_secret);

            let config = TwitchConfig::load();
            assert!(config.is_ok());
            let config = config.unwrap();
            assert_eq!(config.callback_url, data.callback_url);
            assert_eq!(config.client_id, data.client_id);
            assert_eq!(config.client_secret, data.client_secret);
        }
    }
}