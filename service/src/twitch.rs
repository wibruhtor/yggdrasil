use std::sync::Arc;

use twitch_api::TwitchApi;
use types::error::AppResult;
use types::twitch;

pub struct TwitchService {
    twitch_api: Arc<TwitchApi>,
}

impl TwitchService {
    pub fn new(twitch_api: Arc<TwitchApi>) -> Self {
        TwitchService { twitch_api }
    }

    pub async fn get_user_info(&self, login: &str) -> AppResult<twitch::UserInfo> {
        self.twitch_api.get_user_info(login).await
    }

    pub async fn get_global_emotes(&self) -> AppResult<Vec<twitch::Emote>> {
        self.twitch_api.get_global_emotes().await
    }

    pub async fn get_channel_emotes(&self, channel_id: &str) -> AppResult<Vec<twitch::Emote>> {
        self.twitch_api.get_channel_emotes(channel_id).await
    }

    pub async fn get_global_badges(&self) -> AppResult<Vec<twitch::Badge>> {
        self.twitch_api.get_global_badges().await
    }

    pub async fn get_channel_badges(&self, channel_id: &str) -> AppResult<Vec<twitch::Badge>> {
        self.twitch_api.get_channel_badges(channel_id).await
    }
}
