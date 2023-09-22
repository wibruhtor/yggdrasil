use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::{domain::ChatSettings, error::AppResult, jwt::Claims, service::ChatService};

pub async fn handler(
    Extension(chat_service): Extension<Arc<ChatService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<GetChatSettingsPathParams>,
) -> AppResult<Json<ChatSettings>> {
    let chat_settings = chat_service
        .get_chat_settings(&claims.sub, &path_params.chat_settings_id)
        .await?;

    Ok(Json(chat_settings))
}

#[derive(Deserialize)]
pub struct GetChatSettingsPathParams {
    chat_settings_id: Uuid,
}
