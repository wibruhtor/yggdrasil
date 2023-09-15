use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct TwitchData {
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub expired_at: chrono::NaiveDateTime,
}
