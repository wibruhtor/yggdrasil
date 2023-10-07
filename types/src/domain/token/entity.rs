use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub id: Uuid,
    pub user_id: String,
    pub user_agent: String,
    pub ip: String,
    pub authorized_at: NaiveDateTime,
    pub refreshed_at: NaiveDateTime,
}
