use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::AppResult, jwt::Claims, service::ChatService};

pub async fn handler(
    Extension(chat_service): Extension<Arc<ChatService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<DeleteChatSettingsPathParams>,
) -> AppResult<StatusCode> {
    chat_service
        .delete_chat_settings(&claims.sub, &path_params.chat_settings_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct DeleteChatSettingsPathParams {
    chat_settings_id: Uuid,
}
