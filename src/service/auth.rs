use std::sync::Arc;

use crate::{config, dao::user::UserDao};

const AUTHORIZE_URL: &str = "https://id.twitch.tv/oauth2/authorize";
const SCOPE: Vec<&str> = vec![];

#[allow(dead_code)]
pub struct AuthService {
    twitch_config: config::twitch::Twitch,
    user_dao: Arc<UserDao>,
}

#[allow(dead_code)]
impl AuthService {
    pub fn new(twitch_config: config::twitch::Twitch, user_dao: Arc<UserDao>) -> Self {
        AuthService {
            twitch_config,
            user_dao,
        }
    }

    pub fn authorize_url(&self, state: &str) -> String {
        format!(
            "{}?client_id={}&force_verify=true&redirect_uri={}&response_type=code&scope={}&state={}",
            AUTHORIZE_URL, 
            self.twitch_config.client_id,
            self.twitch_config.callback_url,
            SCOPE.join(" "),
            state
        )
    }
}
