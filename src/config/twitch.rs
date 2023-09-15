use std::env;

#[derive(Debug, Clone, PartialEq)]
pub struct Twitch {
    pub callback_url: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Twitch {
    pub fn new() -> Self {
        Twitch {
            callback_url: env::var("TWITCH_CALLBACK_URL").expect("fail get TWITCH_CALLBACK_URL"),
            client_id: env::var("TWITCH_CLIENT_ID").expect("fail get TWITCH_CLIENT_ID"),
            client_secret: env::var("TWITCH_CLIENT_SECRET").expect("fail get TWITCH_CLIENT_SECRET"),
        }
    }
}
