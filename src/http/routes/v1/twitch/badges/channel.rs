use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::Deserialize;

use crate::{domain::TwitchBadge, error::AppResult, service::TwitchService};

pub async fn handler(
    Extension(twitch_service): Extension<Arc<TwitchService>>,
    Path(path_params): Path<GetChannelBadgesPathParams>,
) -> AppResult<Json<Vec<TwitchBadge>>> {
    let emotes = twitch_service
        .get_channel_badges(&path_params.channel_id)
        .await?;

    Ok(Json(emotes))
}

#[derive(Deserialize)]
pub struct GetChannelBadgesPathParams {
    channel_id: String,
}
