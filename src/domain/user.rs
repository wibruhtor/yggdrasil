use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::NaiveDateTime,
}
