use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::{domain::ChatSettings, error::AppResult, jwt::Claims};

pub async fn handler(
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<GetChatSettingsPathParams>,
) -> AppResult<Json<ChatSettings>> {
    todo!()
}

#[derive(Deserialize)]
pub struct GetChatSettingsPathParams {
    chat_settings_id: Uuid,
}
