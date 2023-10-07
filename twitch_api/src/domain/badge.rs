use serde::Deserialize;

use types::twitch;

#[derive(Deserialize, Debug)]
pub struct Badge {
    id: String,
    image_url_4x: String,
}

impl Badge {
    pub fn to_twitch_badge(&self, set: &str) -> twitch::Badge {
        twitch::Badge {
            id: self.id.clone(),
            set: set.to_owned(),
            image: self.image_url_4x.clone(),
        }
    }
}
