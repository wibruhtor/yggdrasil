use std::sync::Arc;

use axum::http::StatusCode;
use sqlx::{Pool, Postgres};

use crate::{
    domain::User,
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

    pub async fn get_or_create(&self, id: &str, username: &str) -> AppResult<User> {
        let span = tracing::debug_span!("get or create user");
        let _span = span.enter();

        match self.get(id).await {
            Ok(user) => Ok(user),
            Err(_) => self.create(id, username).await,
        }
    }

    pub async fn get(&self, id: &str) -> AppResult<User> {
        let span = tracing::debug_span!("get user");
        let _span = span.enter();

        let rec = sqlx::query!(
            r#"SELECT id, username, created_at FROM users WHERE id = $1 LIMIT 1"#,
            id,
        )
        .fetch_one((*self.pool).as_ref())
        .await?;

        Ok(User {
            id: rec.id,
            username: rec.username,
            created_at: rec.created_at,
        })
    }

    async fn create(&self, id: &str, username: &str) -> AppResult<User> {
        let span = tracing::debug_span!("create user");
        let _span = span.enter();

        let rec = sqlx::query!(
            r#"INSERT INTO users (id, username) VALUES ($1, $2) RETURNING id, username, created_at"#,
            id,
            username,
        )
        .fetch_one((*self.pool).as_ref())
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_id_key") => {
                AppError::new(StatusCode::CONFLICT).message("id taken".to_string())
            }
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
