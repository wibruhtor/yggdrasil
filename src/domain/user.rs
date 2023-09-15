use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
}
