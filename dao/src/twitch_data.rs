use std::sync::Arc;

use axum::http::StatusCode;
use sqlx::{Pool, Postgres};
use tracing::instrument;

use types::error::{AppError, AppResult};
use types::twitch;

pub struct TwitchDataDao {
    pool: Arc<Pool<Postgres>>,
}

impl TwitchDataDao {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        TwitchDataDao { pool }
    }

    #[instrument(skip_all)]
    pub async fn create_or_update(
        &self,
        user_id: &str,
        refresh_token: &str,
    ) -> AppResult<twitch::Data> {
        match self.update(user_id, refresh_token).await {
            Ok(twitch_data) => Ok(twitch_data),
            Err(_) => self.create(user_id, refresh_token).await,
        }
    }

    #[instrument(skip_all)]
    async fn create(&self, user_id: &str, refresh_token: &str) -> AppResult<twitch::Data> {
        let raw_twitch_data = sqlx::query_as!(
            RawTwitchData,
            r#"INSERT INTO twitch_data (user_id, refresh_token) VALUES ($1, $2) RETURNING user_id, refresh_token"#,
            user_id,
            refresh_token,
        )
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("twitch_data_user_id_key") => {
                    TwitchDataDao::USER_ID_TAKEN_ERROR
                }
                _ => TwitchDataDao::FAIL_QUERY_ERROR.clone().cause(e.into()),
            })?;

        Ok(raw_twitch_data.into())
    }

    #[instrument(skip_all)]
    async fn update(&self, user_id: &str, refresh_token: &str) -> AppResult<twitch::Data> {
        let raw_twitch_data = sqlx::query_as!(
            RawTwitchData,
            r#"UPDATE twitch_data  SET refresh_token = $1 WHERE user_id = $2 RETURNING user_id, refresh_token"#,
            refresh_token,
            user_id
        )
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| TwitchDataDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        Ok(raw_twitch_data.into())
    }
}

struct RawTwitchData {
    user_id: String,
    refresh_token: String,
}

impl Into<twitch::Data> for RawTwitchData {
    fn into(self) -> twitch::Data {
        twitch::Data {
            user_id: self.user_id,
            refresh_token: self.refresh_token,
        }
    }
}

macro_rules! twitch_data_dao_errors {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $status:expr, $phrase:expr);
        )+
    ) => {
        impl TwitchDataDao {
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

twitch_data_dao_errors! {
    (FAIL_QUERY_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail twitch data query");
    (USER_ID_TAKEN_ERROR, StatusCode::CONFLICT, "user id taken");
}
