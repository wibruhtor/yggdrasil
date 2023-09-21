use std::sync::Arc;

use axum::http::StatusCode;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use tracing::Instrument;
use uuid::Uuid;

use crate::{
    crypt::Crypt,
    domain::Token,
    error::{AppError, AppResult},
};

#[allow(dead_code)]
pub struct TokenDao {
    pool: Arc<Box<Pool<Postgres>>>,
    crypt: Arc<Crypt>,
}

#[allow(dead_code)]
impl TokenDao {
    pub fn new(pool: Arc<Box<Pool<Postgres>>>, crypt: Arc<Crypt>) -> Arc<Self> {
        Arc::new(TokenDao { pool, crypt })
    }

    pub async fn create(&self, user_id: &str, user_agent: &str, ip: &str) -> AppResult<Token> {
        let span = tracing::debug_span!("create token");

        let now = Utc::now().naive_utc();
        let crypted_ip = self.crypt.encrypt_str(&ip);
        let rec = sqlx::query!(
            r#"INSERT INTO tokens (id, user_id, user_agent, ip, authorized_at, refreshed_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, user_id, user_agent, ip, authorized_at, refreshed_at"#,
            Uuid::new_v4(),
            user_id,
            user_agent,
            crypted_ip,
            now,
            now,
        )
        .fetch_one((*self.pool).as_ref())
        .instrument(span)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("tokens_id_key") => {
                AppError::new(StatusCode::CONFLICT).message("id taken".to_string())
            }
            _ => e.into(),
        })?;

        let decrypted_ip = self.crypt.decrypt_str(&rec.ip)?;
        Ok(Token {
            id: rec.id,
            user_id: rec.user_id,
            user_agent: rec.user_agent,
            ip: decrypted_ip,
            authorized_at: rec.authorized_at,
            refreshed_at: rec.refreshed_at,
        })
    }

    pub async fn get(&self, id: &Uuid) -> AppResult<Token> {
        let span = tracing::debug_span!("get token");

        let rec = sqlx::query!(
            r#"SELECT id, user_id, user_agent, ip, authorized_at, refreshed_at FROM tokens WHERE id = $1 LIMIT 1"#,
            id,
        )
        .fetch_one((*self.pool).as_ref())
        .instrument(span)
        .await?;

        let decrypted_ip = self.crypt.decrypt_str(&rec.ip)?;
        Ok(Token {
            id: rec.id,
            user_id: rec.user_id,
            user_agent: rec.user_agent,
            ip: decrypted_ip,
            authorized_at: rec.authorized_at,
            refreshed_at: rec.refreshed_at,
        })
    }

    pub async fn get_all_by_user_id(&self, user_id: &str) -> AppResult<Vec<Token>> {
        let span = tracing::debug_span!("get all tokens by user id");

        let recs = sqlx::query!(
            r#"SELECT id, user_id, user_agent, ip, authorized_at, refreshed_at FROM tokens WHERE user_id = $1"#,
            user_id,
        )
        .fetch_all((*self.pool).as_ref())
        .instrument(span)
        .await?;

        let mut tokens: Vec<Token> = Vec::new();

        for rec in recs {
            let decrypted_ip = self.crypt.decrypt_str(&rec.ip)?;
            tokens.push(Token {
                id: rec.id,
                user_id: rec.user_id,
                user_agent: rec.user_agent,
                ip: decrypted_ip,
                authorized_at: rec.authorized_at,
                refreshed_at: rec.refreshed_at,
            })
        }

        Ok(tokens)
    }

    pub async fn refresh(&self, id: &Uuid) -> AppResult<Token> {
        let span = tracing::debug_span!("refresh token");

        let now = Utc::now().naive_utc();
        let rec = sqlx::query!(
            r#"UPDATE tokens SET refreshed_at = $1 WHERE id = $2 RETURNING id, user_id, user_agent, ip, authorized_at, refreshed_at"#,
            now,
            id
        )
        .fetch_one((*self.pool).as_ref())
        .instrument(span)
        .await?;

        let decrypted_ip = self.crypt.decrypt_str(&rec.ip)?;
        Ok(Token {
            id: rec.id,
            user_id: rec.user_id,
            user_agent: rec.user_agent,
            ip: decrypted_ip,
            authorized_at: rec.authorized_at,
            refreshed_at: rec.refreshed_at,
        })
    }

    pub async fn delete(&self, id: &Uuid) -> AppResult {
        let span = tracing::debug_span!("delete token");

        let rec = sqlx::query!(r#"DELETE FROM tokens WHERE id = $1"#, id,)
            .execute((*self.pool).as_ref())
            .instrument(span)
            .await?;

        if rec.rows_affected() == 0 {
            Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("token not found".to_string()))
        } else {
            Ok(())
        }
    }

    pub async fn delete_with_user_id(&self, id: &Uuid, user_id: &str) -> AppResult {
        let span = tracing::debug_span!("delete token");

        let rec = sqlx::query!(
            r#"DELETE FROM tokens WHERE id = $1 AND user_id = $2"#,
            id,
            user_id
        )
        .execute((*self.pool).as_ref())
        .instrument(span)
        .await?;

        if rec.rows_affected() == 0 {
            Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("token not found".to_string()))
        } else {
            Ok(())
        }
    }

    pub async fn delete_all_by_user_id_exclude_one(&self, user_id: &str, id: &Uuid) -> AppResult {
        let span = tracing::debug_span!("delete all by user id exclude one");

        sqlx::query!(
            r#"DELETE FROM tokens WHERE user_id = $1 AND id != $2"#,
            user_id,
            id
        )
        .execute((*self.pool).as_ref())
        .instrument(span)
        .await?;

        Ok(())
    }
}
