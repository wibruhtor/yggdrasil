use std::sync::Arc;

use sqlx::{Pool, Postgres};
use tokio::signal;

use crate::{config::Config, http::app};

pub async fn run(config: Config, pool: Box<Pool<Postgres>>) {
    let addr = format!("{}:{}", config.http.host, config.http.port);

    tracing::info!("Run server at http://{}", addr);

    let app = app::new(config, Arc::new(pool));

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
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

    tracing::info!("signal received, starting graceful shutdown");
}
