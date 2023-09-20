use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq)]
pub struct Token {
    pub id: Uuid,
    pub user_id: String,
    pub user_agent: String,
    pub ip: String,
    pub authorized_at: chrono::NaiveDateTime,
    pub refreshed_at: chrono::NaiveDateTime,
}
