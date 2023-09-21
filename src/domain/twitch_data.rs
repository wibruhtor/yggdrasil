use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TwitchData {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}
