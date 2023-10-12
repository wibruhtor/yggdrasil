use axum::Router;

// mod ban_word_filters;
mod chat_settings;
mod twitch;

pub fn routes() -> Router {
    Router::new()
        // .nest("/ban-word-filters", ban_word_filters::routes())
        .nest("/chat-settings", chat_settings::routes())
        .nest("/twitch", twitch::routes())
}
