use std::sync::Arc;

use axum::http::StatusCode;
use sqlx::{Pool, Postgres};
use tracing::Instrument;
use uuid::Uuid;

use crate::{
    domain::{
        ChatColorSettings, ChatFontSettings, ChatHideSettings, ChatSettings, ChatSettingsInfo,
        ChatSizeSettings, ChatType, UpdateChatSettings,
    },
    error::{AppError, AppResult},
};

pub struct ChatSettingsDao {
    pool: Arc<Box<Pool<Postgres>>>,
}

impl ChatSettingsDao {
    pub fn new(pool: Arc<Box<Pool<Postgres>>>) -> Arc<Self> {
        Arc::new(ChatSettingsDao { pool })
    }

    pub async fn is_belongs_to_user(&self, id: &Uuid, user_id: &str) -> AppResult<bool> {
        let span = tracing::debug_span!("check user owning of chat settings");

        let rec = sqlx::query!(
            r#"SELECT count(id) FROM chat_settings WHERE id = $1 AND user_id = $2"#,
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

    pub async fn create(
        &self,
        user_id: &str,
        name: &str,
        chat_type: &ChatType,
    ) -> AppResult<ChatSettings> {
        let span = tracing::debug_span!("create chat settings");

        let chat_type = match *chat_type {
            ChatType::Default => "default",
            ChatType::Block => "block",
        };
        let rec = sqlx::query!(
          r#"INSERT INTO chat_settings (id, name, chat_type, user_id) VALUES ($1, $2, $3, $4) RETURNING id, name, chat_type, nickname_color, background_color, text_color, gradient_only_for_custom_nicknames, margin_top, margin_right, margin_bottom, margin_left, padding_top, padding_right, padding_bottom, padding_left, border_top_left_radius, border_top_right_radius, border_bottom_left_radius, border_bottom_right_radius, max_messages, hide_message_pattern, hide_point_rewards, hide_links, link_replacement, ban_word_replacement, ban_word_filter_id, font_family, nickname_font_weight, text_font_weight, font_size, user_id"#,
          Uuid::new_v4(),
          name,
          chat_type,
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

        Ok(ChatSettings {
            id: rec.id,
            name: rec.name,
            chat_type: match rec.chat_type.as_ref() {
                "block" => ChatType::Block,
                _ => ChatType::Default,
            },
            color: ChatColorSettings {
                nickname_color: rec.nickname_color,
                background_color: rec.background_color,
                text_color: rec.text_color,
                gradient_only_for_custom_nicknames: rec.gradient_only_for_custom_nicknames,
            },
            size: ChatSizeSettings {
                margin_top: rec.margin_top,
                margin_right: rec.margin_right,
                margin_bottom: rec.margin_bottom,
                margin_left: rec.margin_left,
                padding_top: rec.padding_top,
                padding_right: rec.padding_right,
                padding_bottom: rec.padding_bottom,
                padding_left: rec.padding_left,
                border_top_left_radius: rec.border_top_left_radius,
                border_top_right_radius: rec.border_top_right_radius,
                border_bottom_left_radius: rec.border_bottom_left_radius,
                border_bottom_right_radius: rec.border_bottom_right_radius,
                max_messages: rec.max_messages,
            },
            hide: ChatHideSettings {
                hide_message_pattern: rec.hide_message_pattern,
                hide_point_rewards: rec.hide_point_rewards,
                hide_links: rec.hide_links,
                link_replacement: rec.link_replacement,
                ban_word_replacement: rec.ban_word_replacement,
                ban_word_filter_id: rec.ban_word_filter_id,
            },
            font: ChatFontSettings {
                font_family: rec.font_family,
                nickname_font_weight: rec.nickname_font_weight,
                text_font_weight: rec.text_font_weight,
                font_size: rec.font_size,
            },
            user_id: rec.user_id,
        })
    }

    pub async fn update(
        &self,
        id: &Uuid,
        update_chat_settings: &UpdateChatSettings,
    ) -> AppResult<ChatSettings> {
        let span = tracing::debug_span!("update chat settings");
        let chat_type = match update_chat_settings.chat_type {
            ChatType::Default => "default",
            ChatType::Block => "block",
        };
        let rec = sqlx::query!(
            r#"UPDATE chat_settings SET (name, chat_type, nickname_color, background_color, text_color, gradient_only_for_custom_nicknames, margin_top, margin_right, margin_bottom, margin_left, padding_top, padding_right, padding_bottom, padding_left, border_top_left_radius, border_top_right_radius, border_bottom_left_radius, border_bottom_right_radius, max_messages, hide_message_pattern, hide_point_rewards, hide_links, link_replacement, ban_word_replacement, ban_word_filter_id, font_family, nickname_font_weight, text_font_weight, font_size) = ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29) WHERE id = $30 RETURNING id, name, chat_type, nickname_color, background_color, text_color, gradient_only_for_custom_nicknames, margin_top, margin_right, margin_bottom, margin_left, padding_top, padding_right, padding_bottom, padding_left, border_top_left_radius, border_top_right_radius, border_bottom_left_radius, border_bottom_right_radius, max_messages, hide_message_pattern, hide_point_rewards, hide_links, link_replacement, ban_word_replacement, ban_word_filter_id, font_family, nickname_font_weight, text_font_weight, font_size, user_id"#,
            update_chat_settings.name,
            chat_type,
            update_chat_settings.color.nickname_color,
            update_chat_settings.color.background_color,
            update_chat_settings.color.text_color,
            update_chat_settings.color.gradient_only_for_custom_nicknames,
            update_chat_settings.size.margin_top,
            update_chat_settings.size.margin_right,
            update_chat_settings.size.margin_bottom,
            update_chat_settings.size.margin_left,
            update_chat_settings.size.padding_top,
            update_chat_settings.size.padding_right,
            update_chat_settings.size.padding_bottom,
            update_chat_settings.size.padding_left,
            update_chat_settings.size.border_top_left_radius,
            update_chat_settings.size.border_top_right_radius,
            update_chat_settings.size.border_bottom_left_radius,
            update_chat_settings.size.border_bottom_right_radius,
            update_chat_settings.size.max_messages,
            update_chat_settings.hide.hide_message_pattern,
            update_chat_settings.hide.hide_point_rewards,
            update_chat_settings.hide.hide_links,
            update_chat_settings.hide.link_replacement,
            update_chat_settings.hide.ban_word_replacement,
            update_chat_settings.hide.ban_word_filter_id,
            update_chat_settings.font.font_family,
            update_chat_settings.font.nickname_font_weight,
            update_chat_settings.font.text_font_weight,
            update_chat_settings.font.font_size,
            id
        )
        .fetch_one((*self.pool).as_ref())
        .instrument(span)
        .await?;

        Ok(ChatSettings {
            id: rec.id,
            name: rec.name,
            chat_type: match rec.chat_type.as_ref() {
                "block" => ChatType::Block,
                _ => ChatType::Default,
            },
            color: ChatColorSettings {
                nickname_color: rec.nickname_color,
                background_color: rec.background_color,
                text_color: rec.text_color,
                gradient_only_for_custom_nicknames: rec.gradient_only_for_custom_nicknames,
            },
            size: ChatSizeSettings {
                margin_top: rec.margin_top,
                margin_right: rec.margin_right,
                margin_bottom: rec.margin_bottom,
                margin_left: rec.margin_left,
                padding_top: rec.padding_top,
                padding_right: rec.padding_right,
                padding_bottom: rec.padding_bottom,
                padding_left: rec.padding_left,
                border_top_left_radius: rec.border_top_left_radius,
                border_top_right_radius: rec.border_top_right_radius,
                border_bottom_left_radius: rec.border_bottom_left_radius,
                border_bottom_right_radius: rec.border_bottom_right_radius,
                max_messages: rec.max_messages,
            },
            hide: ChatHideSettings {
                hide_message_pattern: rec.hide_message_pattern,
                hide_point_rewards: rec.hide_point_rewards,
                hide_links: rec.hide_links,
                link_replacement: rec.link_replacement,
                ban_word_replacement: rec.ban_word_replacement,
                ban_word_filter_id: rec.ban_word_filter_id,
            },
            font: ChatFontSettings {
                font_family: rec.font_family,
                nickname_font_weight: rec.nickname_font_weight,
                text_font_weight: rec.text_font_weight,
                font_size: rec.font_size,
            },
            user_id: rec.user_id,
        })
    }

    pub async fn get(&self, id: &Uuid) -> AppResult<ChatSettings> {
        let span = tracing::debug_span!("get chat settings by id");

        let rec = sqlx::query!(
          r#"SELECT id, name, chat_type, nickname_color, background_color, text_color, gradient_only_for_custom_nicknames, margin_top, margin_right, margin_bottom, margin_left, padding_top, padding_right, padding_bottom, padding_left, border_top_left_radius, border_top_right_radius, border_bottom_left_radius, border_bottom_right_radius, max_messages, hide_message_pattern, hide_point_rewards, hide_links, link_replacement, ban_word_replacement, ban_word_filter_id, font_family, nickname_font_weight, text_font_weight, font_size, user_id FROM chat_settings WHERE id = $1"#,
          id,
      )
      .fetch_one((*self.pool).as_ref())
      .instrument(span)
      .await?;

        Ok(ChatSettings {
            id: rec.id,
            name: rec.name,
            chat_type: match rec.chat_type.as_ref() {
                "block" => ChatType::Block,
                _ => ChatType::Default,
            },
            color: ChatColorSettings {
                nickname_color: rec.nickname_color,
                background_color: rec.background_color,
                text_color: rec.text_color,
                gradient_only_for_custom_nicknames: rec.gradient_only_for_custom_nicknames,
            },
            size: ChatSizeSettings {
                margin_top: rec.margin_top,
                margin_right: rec.margin_right,
                margin_bottom: rec.margin_bottom,
                margin_left: rec.margin_left,
                padding_top: rec.padding_top,
                padding_right: rec.padding_right,
                padding_bottom: rec.padding_bottom,
                padding_left: rec.padding_left,
                border_top_left_radius: rec.border_top_left_radius,
                border_top_right_radius: rec.border_top_right_radius,
                border_bottom_left_radius: rec.border_bottom_left_radius,
                border_bottom_right_radius: rec.border_bottom_right_radius,
                max_messages: rec.max_messages,
            },
            hide: ChatHideSettings {
                hide_message_pattern: rec.hide_message_pattern,
                hide_point_rewards: rec.hide_point_rewards,
                hide_links: rec.hide_links,
                link_replacement: rec.link_replacement,
                ban_word_replacement: rec.ban_word_replacement,
                ban_word_filter_id: rec.ban_word_filter_id,
            },
            font: ChatFontSettings {
                font_family: rec.font_family,
                nickname_font_weight: rec.nickname_font_weight,
                text_font_weight: rec.text_font_weight,
                font_size: rec.font_size,
            },
            user_id: rec.user_id,
        })
    }

    pub async fn get_all_by_user_id(&self, user_id: &str) -> AppResult<Vec<ChatSettings>> {
        let span = tracing::debug_span!("get all chat settings by user id");

        let recs = sqlx::query!(
            r#"SELECT id, name, chat_type, nickname_color, background_color, text_color, gradient_only_for_custom_nicknames, margin_top, margin_right, margin_bottom, margin_left, padding_top, padding_right, padding_bottom, padding_left, border_top_left_radius, border_top_right_radius, border_bottom_left_radius, border_bottom_right_radius, max_messages, hide_message_pattern, hide_point_rewards, hide_links, link_replacement, ban_word_replacement, ban_word_filter_id, font_family, nickname_font_weight, text_font_weight, font_size, user_id FROM chat_settings WHERE user_id = $1"#,
            user_id,
        )
        .fetch_all((*self.pool).as_ref())
        .instrument(span)
        .await?;

        let mut chat_settings: Vec<ChatSettings> = Vec::new();

        for rec in recs {
            chat_settings.push(ChatSettings {
                id: rec.id,
                name: rec.name,
                chat_type: match rec.chat_type.as_ref() {
                    "block" => ChatType::Block,
                    _ => ChatType::Default,
                },
                color: ChatColorSettings {
                    nickname_color: rec.nickname_color,
                    background_color: rec.background_color,
                    text_color: rec.text_color,
                    gradient_only_for_custom_nicknames: rec.gradient_only_for_custom_nicknames,
                },
                size: ChatSizeSettings {
                    margin_top: rec.margin_top,
                    margin_right: rec.margin_right,
                    margin_bottom: rec.margin_bottom,
                    margin_left: rec.margin_left,
                    padding_top: rec.padding_top,
                    padding_right: rec.padding_right,
                    padding_bottom: rec.padding_bottom,
                    padding_left: rec.padding_left,
                    border_top_left_radius: rec.border_top_left_radius,
                    border_top_right_radius: rec.border_top_right_radius,
                    border_bottom_left_radius: rec.border_bottom_left_radius,
                    border_bottom_right_radius: rec.border_bottom_right_radius,
                    max_messages: rec.max_messages,
                },
                hide: ChatHideSettings {
                    hide_message_pattern: rec.hide_message_pattern,
                    hide_point_rewards: rec.hide_point_rewards,
                    hide_links: rec.hide_links,
                    link_replacement: rec.link_replacement,
                    ban_word_replacement: rec.ban_word_replacement,
                    ban_word_filter_id: rec.ban_word_filter_id,
                },
                font: ChatFontSettings {
                    font_family: rec.font_family,
                    nickname_font_weight: rec.nickname_font_weight,
                    text_font_weight: rec.text_font_weight,
                    font_size: rec.font_size,
                },
                user_id: rec.user_id,
            })
        }

        Ok(chat_settings)
    }

    pub async fn get_all_info_by_user_id(&self, user_id: &str) -> AppResult<Vec<ChatSettingsInfo>> {
        let span = tracing::debug_span!("get all chat settings by user id");

        let recs = sqlx::query!(
            r#"SELECT id, name, chat_type, user_id FROM chat_settings WHERE user_id = $1"#,
            user_id,
        )
        .fetch_all((*self.pool).as_ref())
        .instrument(span)
        .await?;

        let mut chat_settings_info: Vec<ChatSettingsInfo> = Vec::new();

        for rec in recs {
            chat_settings_info.push(ChatSettingsInfo {
                id: rec.id,
                name: rec.name,
                chat_type: match rec.chat_type.as_ref() {
                    "block" => ChatType::Block,
                    _ => ChatType::Default,
                },
                user_id: rec.user_id,
            })
        }

        Ok(chat_settings_info)
    }

    pub async fn delete(&self, id: &Uuid) -> AppResult {
        let span = tracing::debug_span!("delete chat settings");

        let rec = sqlx::query!(r#"DELETE FROM chat_settings WHERE id = $1"#, id,)
            .execute((*self.pool).as_ref())
            .instrument(span)
            .await?;

        if rec.rows_affected() == 0 {
            Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("chat settings not found".to_string()))
        } else {
            Ok(())
        }
    }
}
