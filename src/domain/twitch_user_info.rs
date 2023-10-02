use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TwitchUserInfo {
    pub id: String,
    pub login: String,
}
