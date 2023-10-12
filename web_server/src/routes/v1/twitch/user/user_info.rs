use std::sync::Arc;

use axum::{debug_handler, extract::Path, Extension, Json};
use serde::Deserialize;

use service::TwitchService;
use types::error::AppResult;
use types::twitch;

#[debug_handler]
pub async fn handler(
    Extension(twitch_service): Extension<Arc<TwitchService>>,
    Path(path_params): Path<GetTwitchUserInfoPathParams>,
) -> AppResult<Json<twitch::UserInfo>> {
    let twitch_user_info = twitch_service.get_user_info(&path_params.login).await?;

    Ok(Json(twitch_user_info))
}

#[derive(Deserialize)]
pub struct GetTwitchUserInfoPathParams {
    login: String,
}
