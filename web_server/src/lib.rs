use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Server};
use tokio::signal;

use config::HttpConfig;
use service::{AuthService, BanWordService, ChatService, SessionService, TwitchService};
use types::error::AppError;

use crate::routes::routes;

mod routes;

pub struct Services {
    pub auth: Arc<AuthService>,
    pub session: Arc<SessionService>,
    pub twitch: Arc<TwitchService>,
    pub chat: Arc<ChatService>,
    pub ban_word: Arc<BanWordService>,
}

pub async fn run(config: HttpConfig, services: Services) {
    let addr = format!("{}:{}", config.host(), config.port());

    let app = routes()
        .route("/pepega", get(|| async move { AppError::UNEXPECTED }))
        .layer(Extension(services.auth))
        .layer(Extension(services.session))
        .layer(Extension(services.twitch))
        .layer(Extension(services.chat))
        .layer(Extension(services.ban_word))
        .route("/health", get(|| async move { StatusCode::NO_CONTENT }));

    tracing::warn!("listening on http://{}", addr);

    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::warn!("signal received, starting graceful shutdown");
}
