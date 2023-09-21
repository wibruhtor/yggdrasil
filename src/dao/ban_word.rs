use std::sync::Arc;

use sqlx::{Pool, Postgres};
use tracing::Instrument;
use uuid::Uuid;

use crate::error::AppResult;

#[allow(dead_code)]
pub struct BanWordDao {
    pool: Arc<Box<Pool<Postgres>>>,
}

#[allow(dead_code)]
impl BanWordDao {
    pub fn new(pool: Arc<Box<Pool<Postgres>>>) -> Arc<Self> {
        Arc::new(BanWordDao { pool })
    }

    pub async fn get_all_in_filter_by_user_id(
        &self,
        ban_word_filter_id: &Uuid,
        user_id: &str,
    ) -> AppResult<Vec<String>> {
        let span = tracing::debug_span!("get all ban words in filter by user id");

        let recs = sqlx::query!(
            r#"SELECT ban_words.word FROM ban_words INNER JOIN ban_word_filters ON ban_word_filters.id = ban_words.ban_word_filter_id WHERE ban_words.ban_word_filter_id = $1 AND ban_word_filters.user_id = $2"#,
            ban_word_filter_id,
            user_id
        )
        .fetch_all((*self.pool).as_ref())
        .instrument(span)
        .await?;

        let mut words: Vec<String> = Vec::new();

        for rec in recs {
            words.push(rec.word);
        }

        Ok(words)
    }

    pub async fn update_in_filter_by_user_id(
        &self,
        ban_word_filter_id: &Uuid,
        to_create_ban_words: &Vec<String>,
        to_delete_ban_words: &Vec<String>,
    ) -> AppResult {
        let span = tracing::debug_span!("update all ban words in filter by user id");

        let mut tx = self.pool.begin().instrument(span.clone()).await?;

        let ban_word_filter_ids: Vec<Uuid> = to_create_ban_words
            .iter()
            .map(|_| ban_word_filter_id.clone())
            .collect();
        sqlx::query!(
            r#"INSERT INTO ban_words (ban_word_filter_id, word) SELECT * FROM unnest($1::uuid[], $2::text[])"#,
            &ban_word_filter_ids,
            to_create_ban_words
        )
        .execute(&mut *tx)
        .instrument(span.clone())
        .await?;

        sqlx::query!(
            r#"DELETE FROM ban_words WHERE ban_word_filter_id = $1 AND word = any($2::text[]);"#,
            ban_word_filter_id,
            to_delete_ban_words
        )
        .execute(&mut *tx)
        .instrument(span.clone())
        .await?;

        tx.commit().instrument(span).await?;

        Ok(())
    }
}
