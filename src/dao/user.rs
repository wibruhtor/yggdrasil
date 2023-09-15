use std::sync::Arc;

use axum::http::StatusCode;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    domain::user::User,
    error::{AppError, AppResult},
};

#[allow(dead_code)]
pub struct UserDao {
    pool: Arc<Box<Pool<Postgres>>>,
}

#[allow(dead_code)]
impl UserDao {
    pub fn new(pool: Arc<Box<Pool<Postgres>>>) -> Arc<Self> {
        Arc::new(UserDao { pool })
    }

    pub async fn create_user(&self, create_user: CreateUser) -> AppResult<User> {
        let rec = sqlx::query!(
            r#"
INSERT INTO users (id, username)
VALUES ($1, $2)
RETURNING id, username, created_at
        "#,
            Uuid::new_v4(),
            create_user.username,
        )
        .fetch_one((*self.pool).as_ref())
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_username_key") => {
                AppError::new(StatusCode::CONFLICT).message("username taken".to_string())
            }
            _ => e.into(),
        })?;

        Ok(User {
            id: rec.id,
            username: rec.username,
            created_at: rec.created_at,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
}
