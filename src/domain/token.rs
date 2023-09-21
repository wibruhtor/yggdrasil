use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq)]
pub struct Token {
    pub id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    pub ip: String,
    #[serde(rename = "authorizedAt")]
    pub authorized_at: chrono::NaiveDateTime,
    #[serde(rename = "refreshedAt")]
    pub refreshed_at: chrono::NaiveDateTime,
}
