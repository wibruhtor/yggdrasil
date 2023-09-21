use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::NaiveDateTime,
}
