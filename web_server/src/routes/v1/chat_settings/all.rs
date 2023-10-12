use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;

use service::ChatService;
use types::domain::ChatSettingsInfo;
use types::error::AppResult;
use utils::jwt::Claims;

pub async fn handler(
    Extension(chat_service): Extension<Arc<ChatService>>,
    Extension(claims): Extension<Arc<Claims>>,
) -> AppResult<Json<GetAllChatSettingsResponse>> {
    let chat_settings = chat_service.get_all_chat_settings(&claims.sub).await?;

    Ok(Json(GetAllChatSettingsResponse { chat_settings }))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllChatSettingsResponse {
    chat_settings: Vec<ChatSettingsInfo>,
}
