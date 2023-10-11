use std::sync::Arc;

use axum::http::StatusCode;
use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};
use tracing::instrument;

use types::domain::User;
use types::error::{AppError, AppResult};

pub struct UserDao {
    pool: Arc<Pool<Postgres>>,
}

impl UserDao {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        UserDao { pool }
    }

    #[instrument(skip(self))]
    pub async fn get_or_create(&self, id: &str, username: &str) -> AppResult<User> {
        match self.get(id).await {
            Ok(user) => Ok(user),
            Err(_) => self.create(id, username).await,
        }
    }

    #[instrument(skip(self))]
    async fn create(&self, id: &str, username: &str) -> AppResult<User> {
        let raw_user = sqlx::query_as!(
            RawUser,
            r#"INSERT INTO users (id, username) VALUES ($1, $2) RETURNING id, username, created_at"#,
            id,
            username,
        )
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_id_key") => {
                    UserDao::ID_TAKEN_ERROR
                }
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_username_key") => {
                    UserDao::USERNAME_TAKEN_ERROR
                }
                _ => UserDao::FAIL_QUERY_ERROR.clone().cause(e.into()),
            })?;

        Ok(raw_user.into())
    }

    #[instrument(skip(self))]
    pub async fn get(&self, id: &str) -> AppResult<User> {
        let raw_user = sqlx::query_as!(
            RawUser,
            r#"SELECT id, username, created_at FROM users WHERE id = $1 LIMIT 1"#,
            id,
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| UserDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        Ok(raw_user.into())
    }
}

struct RawUser {
    id: String,
    username: String,
    created_at: NaiveDateTime,
}

impl Into<User> for RawUser {
    fn into(self) -> User {
        User {
            id: self.id,
            username: self.username,
            created_at: self.created_at,
        }
    }
}

macro_rules! user_dao_errors {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $status:expr, $phrase:expr);
        )+
    ) => {
        impl UserDao {
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

user_dao_errors! {
    (FAIL_QUERY_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail user query");
    (ID_TAKEN_ERROR, StatusCode::CONFLICT, "id taken");
    (USERNAME_TAKEN_ERROR, StatusCode::CONFLICT, "username taken");
}
