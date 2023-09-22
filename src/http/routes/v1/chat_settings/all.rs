use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;

use crate::{domain::ChatSettings, error::AppResult, jwt::Claims};

pub async fn handler(
    Extension(claims): Extension<Arc<Claims>>,
) -> AppResult<Json<GetAllChatSettingsResponse>> {
    todo!()
}

#[derive(Serialize)]
pub struct GetAllChatSettingsResponse {
    #[serde(rename = "chatSettings")]
    chat_settings: Vec<ChatSettings>,
}
