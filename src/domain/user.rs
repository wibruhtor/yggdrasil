use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
}
