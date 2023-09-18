use std::sync::Arc;

use axum::http::StatusCode;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    domain::Token,
    error::{AppError, AppResult},
};

#[allow(dead_code)]
pub struct TokenDao {
    pool: Arc<Box<Pool<Postgres>>>,
}

#[allow(dead_code)]
impl TokenDao {
    pub fn new(pool: Arc<Box<Pool<Postgres>>>) -> Arc<Self> {
        Arc::new(TokenDao { pool })
    }

    pub async fn create(&self, user_id: &str, user_agent: &str, ip: &str) -> AppResult<Token> {
        let span = tracing::debug_span!("create token");
        let _span = span.enter();

        let now = Utc::now().naive_utc();
        let rec = sqlx::query!(
            r#"INSERT INTO tokens (id, user_id, user_agent, ip, authorized_at, refreshed_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, user_id, user_agent, ip, authorized_at, refreshed_at"#,
            Uuid::new_v4(),
            user_id,
            user_agent,
            ip,
            now,
            now,
        )
        .fetch_one((*self.pool).as_ref())
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("tokens_id_key") => {
                AppError::new(StatusCode::CONFLICT).message("id taken".to_string())
            }
            _ => e.into(),
        })?;

        Ok(Token {
            id: rec.id,
            user_id: rec.user_id,
            user_agent: rec.user_agent,
            ip: rec.ip,
            authorized_at: rec.authorized_at,
            refreshed_at: rec.refreshed_at,
        })
    }

    pub async fn get(&self, id: &Uuid) -> AppResult<Token> {
        let span = tracing::debug_span!("get token");
        let _span = span.enter();

        let rec = sqlx::query!(
            r#"SELECT id, user_id, user_agent, ip, authorized_at, refreshed_at FROM tokens WHERE id = $1 LIMIT 1"#,
            id,
        )
        .fetch_one((*self.pool).as_ref())
        .await?;

        Ok(Token {
            id: rec.id,
            user_id: rec.user_id,
            user_agent: rec.user_agent,
            ip: rec.ip,
            authorized_at: rec.authorized_at,
            refreshed_at: rec.refreshed_at,
        })
    }

    pub async fn refresh(&self, id: &Uuid) -> AppResult<Token> {
        let span = tracing::debug_span!("refresh token");
        let _span = span.enter();

        let now = Utc::now().naive_utc();
        let rec = sqlx::query!(
            r#"UPDATE tokens SET refreshed_at = $1 WHERE id = $2 RETURNING id, user_id, user_agent, ip, authorized_at, refreshed_at"#,
            now,
            id
        )
        .fetch_one((*self.pool).as_ref())
        .await?;

        Ok(Token {
            id: rec.id,
            user_id: rec.user_id,
            user_agent: rec.user_agent,
            ip: rec.ip,
            authorized_at: rec.authorized_at,
            refreshed_at: rec.refreshed_at,
        })
    }

    pub async fn delete(&self, id: &Uuid) -> AppResult {
        let span = tracing::debug_span!("delete token");
        let _span = span.enter();

        let rec = sqlx::query!(r#"DELETE FROM tokens WHERE id = $1"#, id,)
            .execute((*self.pool).as_ref())
            .await?;

        if rec.rows_affected() == 0 {
            Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("token not found".to_string()))
        } else {
            Ok(())
        }
    }
}
