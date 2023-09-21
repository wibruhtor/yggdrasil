use axum::{routing, Json, Router};

use crate::domain::{ChatColorSettings, ChatSettings};

pub fn routes() -> Router {
    Router::new()
        .route("/", routing::get(handler)) // all
        .route("/", routing::post(|| async {})) // create
        .route("/:chat_settings_id", routing::get(|| async {})) // get
        .route("/:chat_settings_id", routing::put(|| async {})) // update
        .route("/:chat_settings_id", routing::delete(|| async {})) // delete
}

async fn handler() -> Json<ChatSettings> {
    Json(ChatSettings {
        id: uuid::Uuid::new_v4(),
        name: "default".to_string(),
        chat_type: crate::domain::ChatType::Block,
        color: ChatColorSettings {
            nickname_color: 3941000703,
            background_color: 303239167,
            text_color: 3941000703,
            gradient_only_for_custom_nicknames: true,
        },
        user_id: "53465346".to_string(),
    })
}
