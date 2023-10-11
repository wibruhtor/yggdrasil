use std::sync::Arc;

use axum::http::StatusCode;
use sqlx::{PgConnection, Pool, Postgres};
use tracing::instrument;
use uuid::Uuid;

use types::domain::{BanWordFilter, BanWordFilterInfo, UpdateBanWordFilter};
use types::error::{AppError, AppResult};

pub struct BanWordFilterDao {
    pool: Arc<Pool<Postgres>>,
}

impl BanWordFilterDao {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        BanWordFilterDao { pool }
    }

    #[instrument(skip(self))]
    pub async fn is_belongs_to_user(&self, id: &Uuid, user_id: &str) -> AppResult<bool> {
        let rec = sqlx::query!(
            r#"SELECT count(id) FROM ban_word_filters WHERE id = $1 AND user_id = $2 LIMIT 1"#,
            id,
            user_id,
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| BanWordFilterDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        let is_belongs_to_user = match rec.count {
            Some(count) => count > 0,
            None => false,
        };

        Ok(is_belongs_to_user)
    }

    #[instrument(skip(self))]
    pub async fn create(&self, user_id: &str, name: &str) -> AppResult<BanWordFilter> {
        let raw_ban_word_filter = sqlx::query_as!(
            RawBanWordFilter,
            r#"INSERT INTO ban_word_filters (id, name, user_id) VALUES ($1, $2, $3) RETURNING id, name, ARRAY[]::varchar[] as ban_words, user_id"#,
            Uuid::new_v4(),
            name,
            user_id,
        )
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("ban_word_filters_id_key") => {
                    BanWordFilterDao::ID_TAKEN_ERROR
                }
                _ => BanWordFilterDao::FAIL_QUERY_ERROR.clone().cause(e.into()),
            })?;

        Ok(raw_ban_word_filter.into())
    }

    #[instrument(skip(self))]
    pub async fn get(&self, id: &Uuid) -> AppResult<BanWordFilter> {
        let raw_ban_word_filter = sqlx::query_as!(
            RawBanWordFilter,
            r#"SELECT f.id, f.name, array_agg(b.word) as ban_words, f.user_id FROM ban_word_filters as f INNER JOIN ban_words as b on f.id = b.ban_word_filter_id WHERE f.id = $1 GROUP by f.id"#,
            id,
        )
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| BanWordFilterDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        Ok(raw_ban_word_filter.into())
    }

    #[instrument(skip(self))]
    pub async fn get_all_by_user_id(&self, user_id: &str) -> AppResult<Vec<BanWordFilterInfo>> {
        let raw_ban_word_filter_infos = sqlx::query_as!(
            RawBanWordFilterInfo,
            r#"SELECT id, name, user_id FROM ban_word_filters WHERE user_id = $1"#,
            user_id,
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| BanWordFilterDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        let mut filters: Vec<BanWordFilterInfo> = Vec::new();

        for raw_ban_word_filter_info in raw_ban_word_filter_infos {
            filters.push(raw_ban_word_filter_info.into());
        }

        Ok(filters)
    }

    #[instrument(skip(self, update_ban_word_filter))]
    pub async fn update(
        &self,
        id: &Uuid,
        update_ban_word_filter: &UpdateBanWordFilter,
    ) -> AppResult<BanWordFilter> {
        let previous_ban_words = self.ban_words(id).await?;

        let mut tx = self.pool.begin().await.map_err(|e| {
            BanWordFilterDao::FAIL_BEGIN_TRANSACTION_ERROR
                .clone()
                .cause(e.into())
        })?;

        // region: create ban words
        let mut to_create_ban_words: Vec<String> = Vec::new();
        for ban_word in update_ban_word_filter.ban_words.clone() {
            if !previous_ban_words.contains(&ban_word) {
                to_create_ban_words.push(ban_word.clone());
            }
        }
        if to_create_ban_words.len() > 0 {
            self.create_ban_words(id, &to_create_ban_words, &mut *tx)
                .await?;
        }
        // endregion

        // region: delete ban words
        let mut to_delete_ban_words: Vec<String> = Vec::new();
        for ban_word in previous_ban_words {
            if !update_ban_word_filter.ban_words.contains(&ban_word) {
                to_delete_ban_words.push(ban_word.clone());
            }
        }
        if to_delete_ban_words.len() > 0 {
            self.delete_ban_words(id, &to_delete_ban_words, &mut *tx)
                .await?;
        }
        // endregion

        let raw_ban_word_filter = sqlx::query_as!(
            RawBanWordFilter,
            r#"with f as (UPDATE ban_word_filters SET name = $1 WHERE id = $2 RETURNING id, name, user_id) select f.id, f.name, array_agg(word) as ban_words, f.user_id from ban_words INNER JOIN f ON ban_words.ban_word_filter_id = f.id where ban_words.ban_word_filter_id = f.id GROUP BY f.id, f.name, f.user_id"#,
            update_ban_word_filter.name,
            id,
        )
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| BanWordFilterDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        tx.commit().await.map_err(|e| {
            BanWordFilterDao::FAIL_COMMIT_TRANSACTION_ERROR
                .clone()
                .cause(e.into())
        })?;

        Ok(raw_ban_word_filter.into())
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: &Uuid) -> AppResult {
        let rec = sqlx::query!(r#"DELETE FROM ban_word_filters WHERE id = $1"#, id,)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| BanWordFilterDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        if rec.rows_affected() == 0 {
            Err(BanWordFilterDao::NOT_FOUND_ERROR)
        } else {
            Ok(())
        }
    }

    #[instrument(skip(self))]
    async fn ban_words(&self, id: &Uuid) -> AppResult<Vec<String>> {
        let recs = sqlx::query!(
            r#"SELECT word FROM ban_words WHERE ban_word_filter_id = $1"#,
            id,
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| BanWordFilterDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        let ban_words: Vec<String> = recs.iter().map(|rec| rec.word.clone()).collect();

        Ok(ban_words)
    }

    #[instrument(skip(self, conn))]
    async fn create_ban_words(
        &self,
        id: &Uuid,
        ban_words: &Vec<String>,
        conn: &mut PgConnection,
    ) -> AppResult {
        sqlx::query!(
            r#"INSERT INTO ban_words (ban_word_filter_id, word) SELECT $1, * FROM unnest($2::varchar[])"#,
            id,
            ban_words
        )
            .execute(conn)
            .await
            .map_err(|e| BanWordFilterDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        Ok(())
    }

    #[instrument(skip(self, conn))]
    async fn delete_ban_words(
        &self,
        id: &Uuid,
        ban_words: &Vec<String>,
        conn: &mut PgConnection,
    ) -> AppResult {
        sqlx::query!(
            r#"DELETE FROM ban_words WHERE ban_word_filter_id = $1 AND word = any($2::varchar[])"#,
            id,
            ban_words
        )
        .execute(conn)
        .await
        .map_err(|e| BanWordFilterDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        Ok(())
    }
}

struct RawBanWordFilter {
    pub id: Uuid,
    pub name: String,
    pub ban_words: Option<Vec<String>>,
    pub user_id: String,
}

impl Into<BanWordFilter> for RawBanWordFilter {
    fn into(self) -> BanWordFilter {
        BanWordFilter {
            id: self.id,
            name: self.name,
            ban_words: match self.ban_words {
                Some(ban_words) => ban_words,
                None => Vec::new(),
            },
            user_id: self.user_id,
        }
    }
}

struct RawBanWordFilterInfo {
    pub id: Uuid,
    pub name: String,
    pub user_id: String,
}

impl Into<BanWordFilterInfo> for RawBanWordFilterInfo {
    fn into(self) -> BanWordFilterInfo {
        BanWordFilterInfo {
            id: self.id,
            name: self.name,
            user_id: self.user_id,
        }
    }
}

macro_rules! ban_word_filter_dao_errors {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $status:expr, $phrase:expr);
        )+
    ) => {
        impl BanWordFilterDao {
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

ban_word_filter_dao_errors! {
    (FAIL_QUERY_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail ban word filter query");
    (FAIL_BEGIN_TRANSACTION_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail begin ban word filter transaction");
    (FAIL_COMMIT_TRANSACTION_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail commit ban word filter transaction");
    (ID_TAKEN_ERROR, StatusCode::CONFLICT, "id taken");
    (NOT_FOUND_ERROR, StatusCode::NOT_FOUND, "ban word filter not found");
}
