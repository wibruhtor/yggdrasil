use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::{ChatSettings, UpdateChatSettings},
    error::{AppResult, ValidationErrorsWrapper},
    jwt::Claims,
    service::ChatService,
};

pub async fn handler(
    Extension(chat_service): Extension<Arc<ChatService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<UpdateChatSettingsPathParams>,
    Json(mut request): Json<UpdateChatSettings>,
) -> AppResult<Json<ChatSettings>> {
    request
        .validate()
        .map_err(|e| ValidationErrorsWrapper::from(e))?;

    let filter = chat_service
        .update_chat_settings(&claims.sub, &path_params.chat_settings_id, &mut request)
        .await?;

    Ok(Json(filter))
}

#[derive(Deserialize)]
pub struct UpdateChatSettingsPathParams {
    chat_settings_id: Uuid,
}
