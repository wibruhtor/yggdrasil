use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, sqlx::FromRow)]
pub struct TwitchData {
    pub user_id: String,
    pub refresh_token: String,
}
