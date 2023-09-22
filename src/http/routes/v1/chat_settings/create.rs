use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{
    domain::{ChatSettings, ChatType},
    error::{AppResult, ValidationErrorsWrapper},
    jwt::Claims,
    service::ChatService,
};

pub async fn handler(
    Extension(chat_service): Extension<Arc<ChatService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Json(request): Json<CreateChatSettingsRequest>,
) -> AppResult<Json<ChatSettings>> {
    request
        .validate()
        .map_err(|e| ValidationErrorsWrapper::from(e))?;

    let chat_settings = chat_service
        .create_chat_settings(&claims.sub, &request.name, &request.chat_type)
        .await?;

    Ok(Json(chat_settings))
}

#[derive(Deserialize, Validate)]
pub struct CreateChatSettingsRequest {
    #[validate(length(min = 2, max = 32))]
    name: String,
    #[serde(rename = "chatType")]
    chat_type: ChatType,
}
