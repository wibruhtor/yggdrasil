use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;

use crate::{domain::ChatSettingsInfo, error::AppResult, jwt::Claims, service::ChatService};

pub async fn handler(
    Extension(chat_service): Extension<Arc<ChatService>>,
    Extension(claims): Extension<Arc<Claims>>,
) -> AppResult<Json<GetAllChatSettingsInfoResponse>> {
    let chat_settings_info = chat_service.get_all_chat_settings_info(&claims.sub).await?;

    Ok(Json(GetAllChatSettingsInfoResponse { chat_settings_info }))
}

#[derive(Serialize)]
pub struct GetAllChatSettingsInfoResponse {
    #[serde(rename = "chatSettingsInfo")]
    chat_settings_info: Vec<ChatSettingsInfo>,
}
