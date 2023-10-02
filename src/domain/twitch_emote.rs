use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TwitchEmote {
    pub id: String,
    pub name: String,
    pub image: String,
}
