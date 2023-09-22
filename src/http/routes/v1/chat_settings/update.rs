use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::{BanWordFilter, ChatSettings, ChatType},
    error::{AppResult, ValidationErrorsWrapper},
    jwt::Claims,
};

pub async fn handler(
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<UpdateChatSettingsPathParams>,
    Json(request): Json<UpdateChatSettingsRequest>,
) -> AppResult<Json<ChatSettings>> {
    request
        .validate()
        .map_err(|e| ValidationErrorsWrapper::from(e))?;

    todo!()
}

#[derive(Deserialize)]
pub struct UpdateChatSettingsPathParams {
    chat_settings_id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct UpdateChatSettingsRequest {
    #[validate(length(min = 2, max = 32))]
    name: String,
    #[serde(rename = "chatType")]
    chat_type: ChatType,
}
