use std::sync::Arc;

use crate::{
    domain::{TwitchEmote, TwitchUserInfo},
    error::AppResult,
    webapi::TwitchWebApi,
};

pub struct TwitchService {
    twitch_web_api: Arc<TwitchWebApi>,
}

impl TwitchService {
    pub fn new(twitch_web_api: Arc<TwitchWebApi>) -> Self {
        TwitchService { twitch_web_api }
    }

    pub async fn get_user_info(&self, login: &str) -> AppResult<TwitchUserInfo> {
        self.twitch_web_api.get_user_info(login).await
    }

    pub async fn get_global_emotes(&self) -> AppResult<Vec<TwitchEmote>> {
        self.twitch_web_api.get_global_emotes().await
    }

    pub async fn get_channel_emotes(&self, channel_id: &str) -> AppResult<Vec<TwitchEmote>> {
        self.twitch_web_api.get_channel_emotes(channel_id).await
    }
}
