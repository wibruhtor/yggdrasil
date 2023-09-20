use std::sync::Arc;

use uuid::Uuid;

use crate::{
    dao::{BanWordDao, BanWordFilterDao},
    domain::BanWordFilter,
    error::AppResult,
};

#[allow(dead_code)]
pub struct BanWordService {
    ban_word_filter_dao: Arc<BanWordFilterDao>,
    ban_word_dao: Arc<BanWordDao>,
}

#[allow(dead_code)]
impl BanWordService {
    pub fn new(ban_word_filter_dao: Arc<BanWordFilterDao>, ban_word_dao: Arc<BanWordDao>) -> Self {
        BanWordService {
            ban_word_filter_dao,
            ban_word_dao,
        }
    }

    pub async fn create_filter(&self, user_id: &str, name: &str) -> AppResult<BanWordFilter> {
        tracing::debug!("create ban word filter");
        self.ban_word_filter_dao.create(user_id, name).await
    }

    pub async fn update_filter(
        &self,
        user_id: &str,
        ban_word_filter_id: &Uuid,
        name: &str,
    ) -> AppResult<BanWordFilter> {
        tracing::debug!("update ban word filter");
        self.ban_word_filter_dao
            .update(ban_word_filter_id, user_id, name)
            .await
    }

    pub async fn get_all_filters(&self, user_id: &str) -> AppResult<Vec<BanWordFilter>> {
        tracing::debug!("get all ban word filters");
        self.ban_word_filter_dao.get_all_by_user_id(user_id).await
    }

    pub async fn get_filter(
        &self,
        user_id: &str,
        ban_word_filter_id: &Uuid,
    ) -> AppResult<(BanWordFilter, Vec<String>)> {
        tracing::debug!("get ban word filter");
        let filter = self
            .ban_word_filter_dao
            .get(ban_word_filter_id, user_id)
            .await?;
        let words = self
            .ban_word_dao
            .get_all_in_filter_by_user_id(ban_word_filter_id, user_id)
            .await?;

        Ok((filter, words))
    }

    pub async fn delete_filter(&self, user_id: &str, ban_word_filter_id: &Uuid) -> AppResult {
        tracing::debug!("delete ban word filter");
        self.ban_word_filter_dao
            .delete(ban_word_filter_id, user_id)
            .await?;

        Ok(())
    }

    pub async fn get_ban_words(
        &self,
        user_id: &str,
        ban_word_filter_id: &Uuid,
    ) -> AppResult<Vec<String>> {
        tracing::debug!("get ban words in filter");
        self.ban_word_dao
            .get_all_in_filter_by_user_id(ban_word_filter_id, user_id)
            .await
    }

    pub async fn update_ban_words(
        &self,
        user_id: &str,
        ban_word_filter_id: &Uuid,
        ban_words: &Vec<String>,
    ) -> AppResult {
        tracing::debug!("get ban words in filter");
        let previous_ban_words = self.get_ban_words(user_id, ban_word_filter_id).await?;

        tracing::debug!("compute to create ban words");
        let mut to_create_ban_words: Vec<String> = Vec::new();
        for ban_word in ban_words {
            if !previous_ban_words.contains(ban_word) {
                to_create_ban_words.push(ban_word.clone());
            }
        }

        tracing::debug!("compute to delete ban words");
        let mut to_delete_ban_words: Vec<String> = Vec::new();
        for ban_word in previous_ban_words {
            if !ban_words.contains(&ban_word) {
                to_delete_ban_words.push(ban_word.clone());
            }
        }

        tracing::debug!("update ban words");
        self.ban_word_dao
            .update_in_filter_by_user_id(
                ban_word_filter_id,
                &to_create_ban_words,
                &to_delete_ban_words,
            )
            .await
    }
}
