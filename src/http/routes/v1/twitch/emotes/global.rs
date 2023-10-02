use std::sync::Arc;

use axum::{Extension, Json};

use crate::{domain::TwitchEmote, error::AppResult, service::TwitchService};

pub async fn handler(
    Extension(twitch_service): Extension<Arc<TwitchService>>,
) -> AppResult<Json<Vec<TwitchEmote>>> {
    let emotes = twitch_service.get_global_emotes().await?;

    Ok(Json(emotes))
}
