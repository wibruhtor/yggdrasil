use uuid::Uuid;

use crate::{
    domain::{ChatSettings, ChatType, UpdateChatSettings},
    error::AppResult,
};

pub struct ChatService {}

impl ChatService {
    pub fn new() -> Self {
        ChatService {}
    }

    pub async fn get_all_chat_settings(&self, user_id: &str) -> AppResult<Vec<ChatSettings>> {
        todo!()
    }

    pub async fn get_chat_settings(
        &self,
        user_id: &str,
        chat_settings_id: &Uuid,
    ) -> AppResult<ChatSettings> {
        todo!()
    }

    pub async fn create_chat_settings(
        &self,
        user_id: &str,
        name: &str,
        chat_type: &ChatType,
    ) -> AppResult<ChatSettings> {
        todo!()
    }

    pub async fn update_chat_settings(
        &self,
        user_id: &str,
        chat_settings_id: &Uuid,
        update_chat_settings: &UpdateChatSettings,
    ) -> AppResult<ChatSettings> {
        todo!()
    }

    pub async fn delete_chat_settings(&self, user_id: &str, chat_settings_id: &Uuid) -> AppResult {
        todo!()
    }
}
