use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TwitchBadge {
    pub id: String,
    pub set: String,
    pub image: String,
}
