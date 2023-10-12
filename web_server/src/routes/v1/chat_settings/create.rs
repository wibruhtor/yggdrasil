use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
use validator::Validate;

use service::ChatService;
use types::domain::{ChatSettings, ChatType};
use types::error::{AppResult, ValidationErrorsWrapper};
use utils::jwt::Claims;

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
#[serde(rename_all = "camelCase")]
pub struct CreateChatSettingsRequest {
    #[validate(length(min = 2, max = 32))]
    name: String,
    chat_type: ChatType,
}
