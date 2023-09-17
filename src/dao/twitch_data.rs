use std::sync::Arc;

use axum::http::StatusCode;
use sqlx::{Pool, Postgres};

use crate::{
    domain::twitch_data::TwitchData,
    error::{AppError, AppResult},
};

#[allow(dead_code)]
pub struct TwitchDataDao {
    pool: Arc<Box<Pool<Postgres>>>,
}

#[allow(dead_code)]
impl TwitchDataDao {
    pub fn new(pool: Arc<Box<Pool<Postgres>>>) -> Arc<Self> {
        Arc::new(TwitchDataDao { pool })
    }

    pub async fn create_or_update(
        &self,
        user_id: &str,
        refresh_token: &str,
    ) -> AppResult<TwitchData> {
        let span = tracing::debug_span!("create or update twitch data");
        let _span = span.enter();

        match self.update(user_id, refresh_token).await {
            Ok(twitch_data) => Ok(twitch_data),
            Err(_) => self.create(user_id, refresh_token).await,
        }
    }

    async fn create(&self, user_id: &str, refresh_token: &str) -> AppResult<TwitchData> {
        let span = tracing::debug_span!("create twitch data");
        let _span = span.enter();

        let rec = sqlx::query!(
            r#"INSERT INTO twitch_data (user_id, refresh_token) VALUES ($1, $2) RETURNING user_id, refresh_token"#,
            user_id,
            refresh_token,
        )
        .fetch_one((*self.pool).as_ref())
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("twitch_data_user_id_key") => {
                AppError::new(StatusCode::CONFLICT).message("user_id taken".to_string())
            }
            _ => e.into(),
        })?;

        Ok(TwitchData {
            user_id: rec.user_id,
            refresh_token: rec.refresh_token,
        })
    }

    async fn update(&self, user_id: &str, refresh_token: &str) -> AppResult<TwitchData> {
        let span = tracing::debug_span!("update twitch data");
        let _span = span.enter();

        let rec = sqlx::query!(
            r#"UPDATE twitch_data  SET refresh_token = $1 WHERE user_id = $2 RETURNING user_id, refresh_token"#,
            refresh_token,
            user_id
        )
        .fetch_one((*self.pool).as_ref())
        .await?;

        Ok(TwitchData {
            user_id: rec.user_id,
            refresh_token: rec.refresh_token,
        })
    }
}
