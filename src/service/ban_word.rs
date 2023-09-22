use std::sync::Arc;

use axum::http::StatusCode;
use uuid::Uuid;

use crate::{
    dao::{BanWordDao, BanWordFilterDao},
    domain::BanWordFilter,
    error::{AppError, AppResult},
};

pub struct BanWordService {
    ban_word_filter_dao: Arc<BanWordFilterDao>,
    ban_word_dao: Arc<BanWordDao>,
}

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
        self.check_user_owning_of_filter_by_id(user_id, ban_word_filter_id)
            .await?;

        tracing::debug!("update ban word filter");
        self.ban_word_filter_dao
            .update(ban_word_filter_id, name)
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
        let filter = self.ban_word_filter_dao.get(ban_word_filter_id).await?;

        self.check_user_owning_of_filter(user_id, &filter).await?;

        let words = self
            .ban_word_dao
            .get_all_in_filter(ban_word_filter_id)
            .await?;

        Ok((filter, words))
    }

    pub async fn delete_filter(&self, user_id: &str, ban_word_filter_id: &Uuid) -> AppResult {
        self.check_user_owning_of_filter_by_id(user_id, ban_word_filter_id)
            .await?;

        tracing::debug!("delete ban word filter");
        self.ban_word_filter_dao.delete(ban_word_filter_id).await?;

        Ok(())
    }

    pub async fn get_ban_words(&self, ban_word_filter_id: &Uuid) -> AppResult<Vec<String>> {
        tracing::debug!("get ban words in filter");
        self.ban_word_dao
            .get_all_in_filter(ban_word_filter_id)
            .await
    }

    pub async fn update_ban_words(
        &self,
        user_id: &str,
        ban_word_filter_id: &Uuid,
        ban_words: &Vec<String>,
    ) -> AppResult {
        self.check_user_owning_of_filter_by_id(user_id, ban_word_filter_id)
            .await?;

        tracing::debug!("get ban words in filter");
        let previous_ban_words = self
            .ban_word_dao
            .get_all_in_filter(ban_word_filter_id)
            .await?;

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
            .update_in_filter(
                ban_word_filter_id,
                &to_create_ban_words,
                &to_delete_ban_words,
            )
            .await
    }

    async fn check_user_owning_of_filter_by_id(
        &self,
        user_id: &str,
        ban_word_filter_id: &Uuid,
    ) -> AppResult {
        tracing::debug!("check user ownings of ban word filter");
        let is_owner = self
            .ban_word_filter_dao
            .is_belongs_to_user(ban_word_filter_id, user_id)
            .await?;

        if !is_owner {
            return Err(AppError::new(StatusCode::UNAUTHORIZED)
                .message("ban word filter is not your".to_string()));
        }

        Ok(())
    }

    async fn check_user_owning_of_filter(
        &self,
        user_id: &str,
        ban_word_filter: &BanWordFilter,
    ) -> AppResult {
        tracing::debug!("check user ownings of ban word filter");
        if ban_word_filter.user_id != user_id {
            return Err(AppError::new(StatusCode::UNAUTHORIZED)
                .message("ban word filter is not your".to_string()));
        }
        Ok(())
    }
}
