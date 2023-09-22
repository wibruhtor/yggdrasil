use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;

use crate::{domain::ChatSettings, error::AppResult, jwt::Claims, service::ChatService};

pub async fn handler(
    Extension(chat_service): Extension<Arc<ChatService>>,
    Extension(claims): Extension<Arc<Claims>>,
) -> AppResult<Json<GetAllChatSettingsResponse>> {
    let chat_settings = chat_service.get_all_chat_settings(&claims.sub).await?;

    Ok(Json(GetAllChatSettingsResponse { chat_settings }))
}

#[derive(Serialize)]
pub struct GetAllChatSettingsResponse {
    #[serde(rename = "chatSettings")]
    chat_settings: Vec<ChatSettings>,
}
