use serde::Deserialize;

use types::twitch;

use crate::domain::Emote;

#[derive(Deserialize, Debug)]
pub struct GetEmotesResponse {
    data: Vec<Emote>,
    template: String,
}

impl GetEmotesResponse {
    pub fn to_twitch_emotes(&self) -> Vec<twitch::Emote> {
        self.data.iter()
            .map(|emote| emote.to_twitch_emote(&self.template))
            .collect()
    }
}