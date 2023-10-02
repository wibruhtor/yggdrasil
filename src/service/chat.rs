use std::sync::Arc;

use itertools::Itertools;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    dao::ChatSettingsDao,
    domain::{ChatSettings, ChatSettingsInfo, ChatType, CustomNickname, UpdateChatSettings},
    error::{AppError, AppResult},
};

pub struct ChatService {
    chat_settings_dao: Arc<ChatSettingsDao>,
}

impl ChatService {
    pub fn new(chat_settings_dao: Arc<ChatSettingsDao>) -> Self {
        ChatService { chat_settings_dao }
    }

    pub async fn get_all_chat_settings(&self, user_id: &str) -> AppResult<Vec<ChatSettings>> {
        tracing::debug!("get all chat settings");
        self.chat_settings_dao.get_all_by_user_id(user_id).await
    }

    pub async fn get_all_chat_settings_info(
        &self,
        user_id: &str,
    ) -> AppResult<Vec<ChatSettingsInfo>> {
        tracing::debug!("get all chat settings info");
        self.chat_settings_dao
            .get_all_info_by_user_id(user_id)
            .await
    }

    pub async fn get_chat_settings(&self, chat_settings_id: &Uuid) -> AppResult<ChatSettings> {
        tracing::debug!("get chat settings");
        self.chat_settings_dao.get(chat_settings_id).await
    }

    pub async fn create_chat_settings(
        &self,
        user_id: &str,
        name: &str,
        chat_type: &ChatType,
    ) -> AppResult<ChatSettings> {
        tracing::debug!("create chat settings");
        self.chat_settings_dao
            .create(user_id, name, chat_type)
            .await
    }

    pub async fn update_chat_settings(
        &self,
        user_id: &str,
        chat_settings_id: &Uuid,
        update_chat_settings: &mut UpdateChatSettings,
    ) -> AppResult<ChatSettings> {
        self.check_user_owning_of_chat_settings_by_id(user_id, chat_settings_id)
            .await?;

        let hidden_nicknames: Vec<String> = update_chat_settings
            .hide
            .nicknames
            .iter()
            .map(|v| v.trim().to_lowercase())
            .unique()
            .collect();
        update_chat_settings.hide.nicknames = hidden_nicknames;
        let custom_nicknames: Vec<CustomNickname> = update_chat_settings
            .color
            .custom_nicknames
            .clone()
            .iter_mut()
            .map(|v| {
                v.nickname = v.nickname.trim().to_lowercase();
                v.clone()
            })
            .unique_by(|v| v.nickname.clone())
            .collect();
        update_chat_settings.color.custom_nicknames = custom_nicknames;

        tracing::debug!("update ban word filter");
        self.chat_settings_dao
            .update(chat_settings_id, update_chat_settings)
            .await
    }

    pub async fn delete_chat_settings(&self, user_id: &str, chat_settings_id: &Uuid) -> AppResult {
        self.check_user_owning_of_chat_settings_by_id(user_id, chat_settings_id)
            .await?;

        tracing::debug!("delete ban word filter");
        self.chat_settings_dao.delete(chat_settings_id).await?;

        Ok(())
    }

    async fn check_user_owning_of_chat_settings_by_id(
        &self,
        user_id: &str,
        ban_word_filter_id: &Uuid,
    ) -> AppResult {
        tracing::debug!("check user ownings of ban word filter");
        let is_owner = self
            .chat_settings_dao
            .is_belongs_to_user(ban_word_filter_id, user_id)
            .await?;

        if !is_owner {
            return Err(AppError::new(StatusCode::UNAUTHORIZED)
                .message("chat settings is not your".to_string()));
        }

        Ok(())
    }
}
