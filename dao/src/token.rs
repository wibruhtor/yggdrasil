use std::sync::Arc;

use axum::http::StatusCode;
use chrono::{NaiveDateTime, Utc};
use sqlx::{Pool, Postgres};
use tracing::instrument;
use uuid::Uuid;

use types::domain::Token;
use types::error::{AppError, AppResult};
use utils::crypt::Crypt;

pub struct TokenDao {
    pool: Arc<Pool<Postgres>>,
    crypt: Crypt,
}

impl TokenDao {
    pub fn new(pool: Arc<Pool<Postgres>>, crypt: Crypt) -> Self {
        TokenDao { pool, crypt }
    }

    #[instrument(skip(self))]
    pub async fn create(&self, user_id: &str, user_agent: &str, ip: &str) -> AppResult<Token> {
        let now = Utc::now().naive_utc();
        let encrypted_ip = self.crypt.encrypt_str(&ip);

        let raw_token = sqlx::query_as!(
            RawToken,
            r#"INSERT INTO tokens (id, user_id, user_agent, ip, authorized_at, refreshed_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, user_id, user_agent, ip, authorized_at, refreshed_at"#,
            Uuid::new_v4(),
            user_id,
            user_agent,
            encrypted_ip,
            now,
            now,
        )
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("tokens_id_key") => {
                    TokenDao::ID_TAKEN_ERROR
                }
                _ => TokenDao::FAIL_QUERY_ERROR.clone().cause(e.into()),
            })?;

        raw_token.into_token(&self.crypt)
    }

    #[instrument(skip(self))]
    pub async fn get(&self, id: &Uuid) -> AppResult<Token> {
        let raw_token = sqlx::query_as!(
            RawToken,
            r#"SELECT id, user_id, user_agent, ip, authorized_at, refreshed_at FROM tokens WHERE id = $1 LIMIT 1"#,
            id,
        )
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| TokenDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        raw_token.into_token(&self.crypt)
    }

    #[instrument(skip(self))]
    pub async fn get_all_by_user_id(&self, user_id: &str) -> AppResult<Vec<Token>> {
        let raw_tokens = sqlx::query_as!(
            RawToken,
            r#"SELECT id, user_id, user_agent, ip, authorized_at, refreshed_at FROM tokens WHERE user_id = $1"#,
            user_id,
        )
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| TokenDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        let mut tokens: Vec<Token> = Vec::new();

        for raw_token in raw_tokens {
            tokens.push(raw_token.into_token(&self.crypt)?)
        }

        Ok(tokens)
    }

    #[instrument(skip(self))]
    pub async fn refresh(&self, id: &Uuid) -> AppResult<Token> {
        let now = Utc::now().naive_utc();
        let raw_token = sqlx::query_as!(
            RawToken,
            r#"UPDATE tokens SET refreshed_at = $1 WHERE id = $2 RETURNING id, user_id, user_agent, ip, authorized_at, refreshed_at"#,
            now,
            id
        )
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| TokenDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        raw_token.into_token(&self.crypt)
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: &Uuid) -> AppResult {
        let rec = sqlx::query!(r#"DELETE FROM tokens WHERE id = $1"#, id,)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| TokenDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        if rec.rows_affected() == 0 {
            Err(TokenDao::NOT_FOUND_ERROR)
        } else {
            Ok(())
        }
    }

    #[instrument(skip(self))]
    pub async fn delete_with_user_id(&self, id: &Uuid, user_id: &str) -> AppResult {
        let rec = sqlx::query!(
            r#"DELETE FROM tokens WHERE id = $1 AND user_id = $2"#,
            id,
            user_id
        )
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| TokenDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        if rec.rows_affected() == 0 {
            Err(TokenDao::NOT_FOUND_ERROR)
        } else {
            Ok(())
        }
    }

    #[instrument(skip(self))]
    pub async fn delete_all_by_user_id_exclude_one(&self, user_id: &str, id: &Uuid) -> AppResult {
        sqlx::query!(
            r#"DELETE FROM tokens WHERE user_id = $1 AND id != $2"#,
            user_id,
            id
        )
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| TokenDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        Ok(())
    }
}

struct RawToken {
    pub id: Uuid,
    pub user_id: String,
    pub user_agent: String,
    pub ip: String,
    pub authorized_at: NaiveDateTime,
    pub refreshed_at: NaiveDateTime,
}

impl RawToken {
    fn into_token(self, crypt: &Crypt) -> AppResult<Token> {
        Ok(Token {
            id: self.id,
            user_id: self.user_id,
            user_agent: self.user_agent,
            ip: crypt.decrypt_str(&self.ip)?,
            authorized_at: self.authorized_at,
            refreshed_at: self.refreshed_at,
        })
    }
}

macro_rules! token_dao_errors {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $status:expr, $phrase:expr);
        )+
    ) => {
        impl TokenDao {
        $(
            $(#[$docs])*
            pub const $name: AppError = AppError {
                status_code: $status,
                message: Some($phrase),
                cause: None,
                other: None
            };
        )+
        }
    }
}

token_dao_errors! {
    (FAIL_QUERY_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail token query");
    (ID_TAKEN_ERROR, StatusCode::CONFLICT, "id taken");
    (NOT_FOUND_ERROR, StatusCode::NOT_FOUND, "token not found");
}
