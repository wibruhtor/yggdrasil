use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::Deserialize;

use service::TwitchService;
use types::error::AppResult;
use types::twitch;

pub async fn handler(
    Extension(twitch_service): Extension<Arc<TwitchService>>,
    Path(path_params): Path<GetChannelEmotesPathParams>,
) -> AppResult<Json<Vec<twitch::Emote>>> {
    let emotes = twitch_service
        .get_channel_emotes(&path_params.channel_id)
        .await?;

    Ok(Json(emotes))
}

#[derive(Deserialize)]
pub struct GetChannelEmotesPathParams {
    channel_id: String,
}
