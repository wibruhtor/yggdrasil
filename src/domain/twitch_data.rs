use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, sqlx::FromRow)]
pub struct TwitchData {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}
