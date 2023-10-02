use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::Deserialize;

use crate::{domain::TwitchUserInfo, error::AppResult, service::TwitchService};

pub async fn handler(
    Extension(twitch_service): Extension<Arc<TwitchService>>,
    Path(path_params): Path<GetTwitchUserInfoPathParams>,
) -> AppResult<Json<TwitchUserInfo>> {
    let twitch_user_info = twitch_service.get_user_info(&path_params.login).await?;

    Ok(Json(twitch_user_info))
}

#[derive(Deserialize)]
pub struct GetTwitchUserInfoPathParams {
    login: String,
}
