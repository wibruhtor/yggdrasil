use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::AppResult, jwt::Claims};

pub async fn handler(
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<DeleteChatSettingsPathParams>,
) -> AppResult<StatusCode> {
    todo!()
}

#[derive(Deserialize)]
pub struct DeleteChatSettingsPathParams {
    chat_settings_id: Uuid,
}
