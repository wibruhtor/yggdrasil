use std::sync::Arc;

use axum::http::StatusCode;
use sqlx::{PgConnection, Pool, Postgres};
use uuid::Uuid;

use types::domain::{
    ChatColorSettings, ChatFontSettings, ChatHideSettings, ChatSettings, ChatSettingsInfo,
    ChatSizeSettings, ChatType, CustomNickname, UpdateChatSettings,
};
use types::error::{AppError, AppResult};

pub struct ChatSettingsDao {
    pool: Arc<Pool<Postgres>>,
}

impl ChatSettingsDao {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        ChatSettingsDao { pool }
    }

    pub async fn is_belongs_to_user(&self, id: &Uuid, user_id: &str) -> AppResult<bool> {
        let rec = sqlx::query!(
            r#"SELECT count(id) FROM chat_settings WHERE id = $1 AND user_id = $2 LIMIT 1"#,
            id,
            user_id,
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

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
        let raw_chat_settings = sqlx::query_as!(
            RawChatSettings,
            r#"INSERT INTO chat_settings (id, name, chat_type, user_id) VALUES ($1, $2, $3, $4) RETURNING id, name, chat_type, nickname_color, background_color, text_color, gradient_only_for_custom_nicknames, margin_top, margin_right, margin_bottom, margin_left, padding_top, padding_right, padding_bottom, padding_left, border_top_left_radius, border_top_right_radius, border_bottom_left_radius, border_bottom_right_radius, max_messages, hide_message_pattern, hide_point_rewards, hide_links, link_replacement, ban_word_replacement, ban_word_filter_id, font_family, nickname_font_weight, text_font_weight, font_size, user_id"#,
            Uuid::new_v4(),
            name,
            chat_type.to_str(),
            user_id,
        )
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("chat_settings_id_key") => {
                    ChatSettingsDao::ID_TAKEN_ERROR
                }
                _ => ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()),
            })?;

        Ok(raw_chat_settings.into())
    }

    pub async fn get(&self, id: &Uuid) -> AppResult<ChatSettings> {
        let raw_chat_settings = sqlx::query_as!(
            RawChatSettings,
            r#"SELECT id, name, chat_type, nickname_color, background_color, text_color, gradient_only_for_custom_nicknames, margin_top, margin_right, margin_bottom, margin_left, padding_top, padding_right, padding_bottom, padding_left, border_top_left_radius, border_top_right_radius, border_bottom_left_radius, border_bottom_right_radius, max_messages, hide_message_pattern, hide_point_rewards, hide_links, link_replacement, ban_word_replacement, ban_word_filter_id, font_family, nickname_font_weight, text_font_weight, font_size, user_id FROM chat_settings WHERE id = $1"#,
            id,
        )
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        let custom_nicknames = self.get_custom_nicknames(id).await?;
        let hidden_nicknames = self.get_hidden_nicknames(id).await?;

        let mut chat_settings: ChatSettings = raw_chat_settings.into();

        chat_settings.color.custom_nicknames = custom_nicknames;
        chat_settings.hide.nicknames = hidden_nicknames;

        Ok(chat_settings)
    }

    pub async fn get_all_info_by_user_id(&self, user_id: &str) -> AppResult<Vec<ChatSettingsInfo>> {
        let raw_chat_settings_infos = sqlx::query_as!(
            RawChatSettingsInfo,
            r#"SELECT id, name, chat_type, user_id FROM chat_settings WHERE user_id = $1"#,
            user_id,
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        let mut chat_settings_infos: Vec<ChatSettingsInfo> = Vec::new();

        for raw_chat_settings_info in raw_chat_settings_infos {
            chat_settings_infos.push(raw_chat_settings_info.into());
        }

        Ok(chat_settings_infos)
    }

    pub async fn update(
        &self,
        id: &Uuid,
        update_chat_settings: &UpdateChatSettings,
    ) -> AppResult<ChatSettings> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            ChatSettingsDao::FAIL_BEGIN_TRANSACTION_ERROR
                .clone()
                .cause(e.into())
        })?;

        let mut custom_nicknames: Vec<CustomNickname> = Vec::new();

        for nickname in update_chat_settings.color.custom_nicknames.clone() {
            let custom_nickname = nickname.into();
            custom_nicknames.push(custom_nickname);
        }
        self.calculate_and_update_custom_nicknames(id, &custom_nicknames, &mut *tx)
            .await?;

        self.calculate_and_update_hidden_nicknames(
            id,
            &update_chat_settings.hide.nicknames,
            &mut *tx,
        )
        .await?;

        let raw_chat_settings = sqlx::query_as!(
            RawChatSettings,
            r#"UPDATE chat_settings SET (name, chat_type, nickname_color, background_color, text_color, gradient_only_for_custom_nicknames, margin_top, margin_right, margin_bottom, margin_left, padding_top, padding_right, padding_bottom, padding_left, border_top_left_radius, border_top_right_radius, border_bottom_left_radius, border_bottom_right_radius, max_messages, hide_message_pattern, hide_point_rewards, hide_links, link_replacement, ban_word_replacement, ban_word_filter_id, font_family, nickname_font_weight, text_font_weight, font_size) = ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29) WHERE id = $30 RETURNING id, name, chat_type, nickname_color, background_color, text_color, gradient_only_for_custom_nicknames, margin_top, margin_right, margin_bottom, margin_left, padding_top, padding_right, padding_bottom, padding_left, border_top_left_radius, border_top_right_radius, border_bottom_left_radius, border_bottom_right_radius, max_messages, hide_message_pattern, hide_point_rewards, hide_links, link_replacement, ban_word_replacement, ban_word_filter_id, font_family, nickname_font_weight, text_font_weight, font_size, user_id"#,
            update_chat_settings.name,
            update_chat_settings.chat_type.to_str(),
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
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        tx.commit().await.map_err(|e| {
            ChatSettingsDao::FAIL_COMMIT_TRANSACTION_ERROR
                .clone()
                .cause(e.into())
        })?;

        let mut chat_settings: ChatSettings = raw_chat_settings.into();

        chat_settings.color.custom_nicknames = custom_nicknames;
        chat_settings.hide.nicknames = update_chat_settings.hide.nicknames.clone();

        Ok(chat_settings)
    }

    pub async fn delete(&self, id: &Uuid) -> AppResult {
        let rec = sqlx::query!(r#"DELETE FROM chat_settings WHERE id = $1"#, id,)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        if rec.rows_affected() == 0 {
            Err(ChatSettingsDao::NOT_FOUND_ERROR)
        } else {
            Ok(())
        }
    }

    async fn get_custom_nicknames(&self, id: &Uuid) -> AppResult<Vec<CustomNickname>> {
        let raw_custom_nicknames = sqlx::query_as!(
            RawCustomNickname,
            r#"SELECT nickname, start_color, end_color FROM chat_custom_nicknames WHERE chat_settings_id = $1"#,
            id,
        )
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        let mut custom_nicknames: Vec<CustomNickname> = Vec::new();

        for raw_custom_nickname in raw_custom_nicknames {
            custom_nicknames.push(raw_custom_nickname.into());
        }

        Ok(custom_nicknames)
    }

    async fn get_hidden_nicknames(&self, id: &Uuid) -> AppResult<Vec<String>> {
        let recs = sqlx::query!(
            r#"SELECT nickname FROM chat_hidden_nicknames WHERE chat_settings_id = $1"#,
            id,
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        let mut nicknames: Vec<String> = Vec::new();

        for rec in recs {
            nicknames.push(rec.nickname);
        }

        Ok(nicknames)
    }

    async fn calculate_and_update_custom_nicknames(
        &self,
        id: &Uuid,
        custom_nicknames: &Vec<CustomNickname>,
        conn: &mut PgConnection,
    ) -> AppResult {
        let nicknames: Vec<String> = custom_nicknames
            .iter()
            .map(|c| c.nickname.clone())
            .collect();
        let previous_custom_nicknames = self.get_custom_nicknames(id).await?;
        let previous_nicknames: Vec<String> = previous_custom_nicknames
            .iter()
            .map(|c| c.nickname.clone())
            .collect();

        // region: create custom nicknames
        let mut to_create: Vec<CustomNickname> = Vec::new();
        for custom_nickname in custom_nicknames.clone() {
            if !previous_nicknames.contains(&custom_nickname.nickname) {
                to_create.push(custom_nickname.clone());
            }
        }
        if to_create.len() > 0 {
            self.create_custom_nicknames(id, &to_create, conn).await?;
        }
        // endregion

        // region: Update custom nicknames
        let mut to_update: Vec<CustomNickname> = Vec::new();
        for custom_nickname in custom_nicknames.clone() {
            if previous_nicknames.contains(&custom_nickname.nickname) {
                to_update.push(custom_nickname.clone());
            }
        }
        if to_update.len() > 0 {
            self.update_custom_nicknames(id, &to_update, conn).await?;
        }
        // endregion

        // region: Delete custom nicknames
        let mut to_delete: Vec<CustomNickname> = Vec::new();
        for previous in previous_custom_nicknames.clone() {
            if !nicknames.contains(&previous.nickname) {
                to_delete.push(previous.clone());
            }
        }
        if to_delete.len() > 0 {
            self.delete_custom_nicknames(id, &to_delete, conn).await?;
        }
        // endregion

        Ok(())
    }

    async fn create_custom_nicknames(
        &self,
        id: &Uuid,
        custom_nicknames: &Vec<CustomNickname>,
        conn: &mut PgConnection,
    ) -> AppResult {
        let nicknames: Vec<String> = custom_nicknames
            .iter()
            .map(|v| v.nickname.clone())
            .collect();
        let start_colors: Vec<i64> = custom_nicknames.iter().map(|v| v.start_color).collect();
        let end_colors: Vec<i64> = custom_nicknames.iter().map(|v| v.end_color).collect();
        sqlx::query!(
            r#"INSERT INTO chat_custom_nicknames (chat_settings_id, nickname, start_color, end_color) SELECT $1, * FROM unnest($2::varchar[], $3::bigint[], $4::bigint[])"#,
            id,
            &nicknames,
            &start_colors,
            &end_colors
        )
            .execute(conn)
            .await
            .map_err(|e| ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        Ok(())
    }

    async fn update_custom_nicknames(
        &self,
        id: &Uuid,
        custom_nicknames: &Vec<CustomNickname>,
        conn: &mut PgConnection,
    ) -> AppResult {
        let nicknames: Vec<String> = custom_nicknames
            .iter()
            .map(|v| v.nickname.clone())
            .collect();
        let start_colors: Vec<i64> = custom_nicknames.iter().map(|v| v.start_color).collect();
        let end_colors: Vec<i64> = custom_nicknames.iter().map(|v| v.end_color).collect();
        sqlx::query!(
            r#"UPDATE chat_custom_nicknames SET start_color = bulk_query.start_color, end_color = bulk_query.end_color FROM (SELECT * FROM unnest($2::varchar[], $3::bigint[], $4::bigint[]) as t(nickname, start_color, end_color)) as bulk_query WHERE chat_custom_nicknames.chat_settings_id = $1 AND chat_custom_nicknames.nickname = bulk_query.nickname"#,
            id,
            &nicknames,
            &start_colors,
            &end_colors
        )
            .execute(conn)
            .await
            .map_err(|e| ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        Ok(())
    }

    async fn delete_custom_nicknames(
        &self,
        id: &Uuid,
        custom_nicknames: &Vec<CustomNickname>,
        conn: &mut PgConnection,
    ) -> AppResult {
        let nicknames: Vec<String> = custom_nicknames
            .iter()
            .map(|v| v.nickname.clone())
            .collect();
        sqlx::query!(
            r#"DELETE FROM chat_custom_nicknames WHERE chat_settings_id = $1 AND nickname = any($2::varchar[])"#,
            id,
            &nicknames,
        )
            .execute(conn)
            .await
            .map_err(|e| ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        Ok(())
    }
    async fn calculate_and_update_hidden_nicknames(
        &self,
        id: &Uuid,
        hidden_nicknames: &Vec<String>,
        conn: &mut PgConnection,
    ) -> AppResult {
        let previous_hidden_nicknames: Vec<String> = self.get_hidden_nicknames(id).await?;

        // region: create hidden nicknames
        let mut to_create: Vec<String> = Vec::new();
        for nickname in hidden_nicknames.clone() {
            if !previous_hidden_nicknames.contains(&nickname) {
                to_create.push(nickname.clone());
            }
        }
        if to_create.len() > 0 {
            self.create_hidden_nicknames(id, &to_create, conn).await?;
        }
        // endregion

        // region: Delete hidden nicknames
        let mut to_delete: Vec<String> = Vec::new();
        for previous in previous_hidden_nicknames.clone() {
            if !hidden_nicknames.contains(&previous) {
                to_delete.push(previous.clone());
            }
        }
        if to_delete.len() > 0 {
            self.delete_hidden_nicknames(id, &to_delete, conn).await?;
        }
        // endregion

        Ok(())
    }

    async fn create_hidden_nicknames(
        &self,
        id: &Uuid,
        hidden_nicknames: &Vec<String>,
        conn: &mut PgConnection,
    ) -> AppResult {
        sqlx::query!(
            r#"INSERT INTO chat_hidden_nicknames (chat_settings_id, nickname) SELECT $1, * FROM unnest($2::varchar[])"#,
            id,
            &hidden_nicknames,
        )
            .execute(conn)
            .await
            .map_err(|e| ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        Ok(())
    }

    async fn delete_hidden_nicknames(
        &self,
        id: &Uuid,
        hidden_nicknames: &Vec<String>,
        conn: &mut PgConnection,
    ) -> AppResult {
        sqlx::query!(
            r#"DELETE FROM chat_hidden_nicknames WHERE chat_settings_id = $1 AND nickname = any($2::varchar[])"#,
            id,
            &hidden_nicknames,
        )
            .execute(conn)
            .await
            .map_err(|e| ChatSettingsDao::FAIL_QUERY_ERROR.clone().cause(e.into()))?;

        Ok(())
    }
}

struct RawChatSettings {
    id: Uuid,
    name: String,
    chat_type: String,
    nickname_color: i64,
    background_color: i64,
    text_color: i64,
    gradient_only_for_custom_nicknames: bool,
    margin_top: f64,
    margin_right: f64,
    margin_bottom: f64,
    margin_left: f64,
    padding_top: f64,
    padding_right: f64,
    padding_bottom: f64,
    padding_left: f64,
    border_top_left_radius: f64,
    border_top_right_radius: f64,
    border_bottom_left_radius: f64,
    border_bottom_right_radius: f64,
    max_messages: i32,
    hide_message_pattern: String,
    hide_point_rewards: bool,
    hide_links: bool,
    link_replacement: String,
    ban_word_replacement: String,
    ban_word_filter_id: Option<Uuid>,
    font_family: String,
    nickname_font_weight: i32,
    text_font_weight: i32,
    font_size: f64,
    user_id: String,
}

impl Into<ChatSettings> for RawChatSettings {
    fn into(self) -> ChatSettings {
        ChatSettings {
            id: self.id,
            name: self.name,
            chat_type: ChatType::from_str(&self.chat_type),
            color: ChatColorSettings {
                nickname_color: self.nickname_color,
                background_color: self.background_color,
                text_color: self.text_color,
                gradient_only_for_custom_nicknames: self.gradient_only_for_custom_nicknames,
                custom_nicknames: Vec::new(),
            },
            size: ChatSizeSettings {
                margin_top: self.margin_top,
                margin_right: self.margin_right,
                margin_bottom: self.margin_bottom,
                margin_left: self.margin_left,
                padding_top: self.padding_top,
                padding_right: self.padding_right,
                padding_bottom: self.padding_bottom,
                padding_left: self.padding_left,
                border_top_left_radius: self.border_top_left_radius,
                border_top_right_radius: self.border_top_right_radius,
                border_bottom_left_radius: self.border_bottom_left_radius,
                border_bottom_right_radius: self.border_bottom_right_radius,
                max_messages: self.max_messages,
            },
            hide: ChatHideSettings {
                hide_message_pattern: self.hide_message_pattern,
                hide_point_rewards: self.hide_point_rewards,
                hide_links: self.hide_links,
                link_replacement: self.link_replacement,
                ban_word_replacement: self.ban_word_replacement,
                ban_word_filter_id: self.ban_word_filter_id,
                nicknames: Vec::new(),
            },
            font: ChatFontSettings {
                font_family: self.font_family,
                nickname_font_weight: self.nickname_font_weight,
                text_font_weight: self.text_font_weight,
                font_size: self.font_size,
            },
            user_id: self.user_id,
        }
    }
}

struct RawChatSettingsInfo {
    id: Uuid,
    name: String,
    chat_type: String,
    user_id: String,
}

impl Into<ChatSettingsInfo> for RawChatSettingsInfo {
    fn into(self) -> ChatSettingsInfo {
        ChatSettingsInfo {
            id: self.id,
            name: self.name,
            chat_type: ChatType::from_str(&self.chat_type),
            user_id: self.user_id,
        }
    }
}

struct RawCustomNickname {
    nickname: String,
    start_color: i64,
    end_color: i64,
}

impl Into<CustomNickname> for RawCustomNickname {
    fn into(self) -> CustomNickname {
        CustomNickname {
            nickname: self.nickname,
            start_color: self.start_color,
            end_color: self.end_color,
        }
    }
}

macro_rules! chat_settings_dao_errors {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $status:expr, $phrase:expr);
        )+
    ) => {
        impl ChatSettingsDao {
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

chat_settings_dao_errors! {
    (FAIL_QUERY_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail chat settings query");
    (FAIL_BEGIN_TRANSACTION_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail begin chat settings transaction");
    (FAIL_COMMIT_TRANSACTION_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail commit chat settings transaction");
    (ID_TAKEN_ERROR, StatusCode::CONFLICT, "id taken");
    (NOT_FOUND_ERROR, StatusCode::NOT_FOUND, "chat settings not found");
}
