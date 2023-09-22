use std::sync::Arc;

use axum::http::StatusCode;
use sqlx::{Pool, Postgres};
use tracing::Instrument;
use uuid::Uuid;

use crate::{
    domain::BanWordFilter,
    error::{AppError, AppResult},
};

pub struct BanWordFilterDao {
    pool: Arc<Box<Pool<Postgres>>>,
}

impl BanWordFilterDao {
    pub fn new(pool: Arc<Box<Pool<Postgres>>>) -> Arc<Self> {
        Arc::new(BanWordFilterDao { pool })
    }

    pub async fn is_belongs_to_user(&self, id: &Uuid, user_id: &str) -> AppResult<bool> {
        let span = tracing::debug_span!("check user owning of ban word filter");

        let rec = sqlx::query!(
            r#"SELECT count(id) FROM ban_word_filters WHERE id = $1 AND user_id = $2"#,
            id,
            user_id,
        )
        .fetch_one((*self.pool).as_ref())
        .instrument(span)
        .await?;

        let is_belongs_to_user = match rec.count {
            Some(count) => count > 0,
            None => false,
        };

        Ok(is_belongs_to_user)
    }

    pub async fn create(&self, user_id: &str, name: &str) -> AppResult<BanWordFilter> {
        let span = tracing::debug_span!("create ban word filter");

        let rec = sqlx::query!(
            r#"INSERT INTO ban_word_filters (id, name, user_id) VALUES ($1, $2, $3) RETURNING id, name, user_id"#,
            Uuid::new_v4(),
            name,
            user_id,
        )
        .fetch_one((*self.pool).as_ref())
        .instrument(span)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("ban_word_filters_id_key") => {
                AppError::new(StatusCode::CONFLICT).message("id taken".to_string())
            }
            _ => e.into(),
        })?;

        Ok(BanWordFilter {
            id: rec.id,
            name: rec.name,
            user_id: rec.user_id,
        })
    }

    pub async fn update(&self, id: &Uuid, name: &str) -> AppResult<BanWordFilter> {
        let span = tracing::debug_span!("update ban word filter");

        let rec = sqlx::query!(
            r#"UPDATE ban_word_filters SET name = $1 WHERE id = $2 RETURNING id, name, user_id"#,
            name,
            id,
        )
        .fetch_one((*self.pool).as_ref())
        .instrument(span)
        .await?;

        Ok(BanWordFilter {
            id: rec.id,
            name: rec.name,
            user_id: rec.user_id,
        })
    }

    pub async fn get(&self, id: &Uuid) -> AppResult<BanWordFilter> {
        let span = tracing::debug_span!("get ban word filter by id");

        let rec = sqlx::query!(
            r#"SELECT id, name, user_id FROM ban_word_filters WHERE id = $1"#,
            id,
        )
        .fetch_one((*self.pool).as_ref())
        .instrument(span)
        .await?;

        Ok(BanWordFilter {
            id: rec.id,
            name: rec.name,
            user_id: rec.user_id,
        })
    }

    pub async fn get_all_by_user_id(&self, user_id: &str) -> AppResult<Vec<BanWordFilter>> {
        let span = tracing::debug_span!("get all ban word filters by user id");

        let recs = sqlx::query!(
            r#"SELECT id, name, user_id FROM ban_word_filters WHERE user_id = $1"#,
            user_id,
        )
        .fetch_all((*self.pool).as_ref())
        .instrument(span)
        .await?;

        let mut filters: Vec<BanWordFilter> = Vec::new();

        for rec in recs {
            filters.push(BanWordFilter {
                id: rec.id,
                name: rec.name,
                user_id: rec.user_id,
            })
        }

        Ok(filters)
    }

    pub async fn delete(&self, id: &Uuid) -> AppResult {
        let span = tracing::debug_span!("delete ban word filter");

        let rec = sqlx::query!(r#"DELETE FROM ban_word_filters WHERE id = $1"#, id,)
            .execute((*self.pool).as_ref())
            .instrument(span)
            .await?;

        if rec.rows_affected() == 0 {
            Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("ban word filter not found".to_string()))
        } else {
            Ok(())
        }
    }
}
