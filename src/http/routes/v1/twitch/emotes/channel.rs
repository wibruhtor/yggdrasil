use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::Deserialize;

use crate::{domain::TwitchEmote, error::AppResult, service::TwitchService};

pub async fn handler(
    Extension(twitch_service): Extension<Arc<TwitchService>>,
    Path(path_params): Path<GetChannelEmotesPathParams>,
) -> AppResult<Json<Vec<TwitchEmote>>> {
    let emotes = twitch_service
        .get_channel_emotes(&path_params.channel_id)
        .await?;

    Ok(Json(emotes))
}

#[derive(Deserialize)]
pub struct GetChannelEmotesPathParams {
    channel_id: String,
}
