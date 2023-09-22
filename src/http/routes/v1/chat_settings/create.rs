use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{
    domain::{BanWordFilter, ChatSettings, ChatType},
    error::{AppResult, ValidationErrorsWrapper},
    jwt::Claims,
};

pub async fn handler(
    Extension(claims): Extension<Arc<Claims>>,
    Json(request): Json<CreateChatSettingsRequest>,
) -> AppResult<Json<ChatSettings>> {
    request
        .validate()
        .map_err(|e| ValidationErrorsWrapper::from(e))?;

    todo!()
}

#[derive(Deserialize, Validate)]
pub struct CreateChatSettingsRequest {
    #[validate(length(min = 2, max = 32))]
    name: String,
    #[serde(rename = "chatType")]
    chat_type: ChatType,
}
