use axum::Router;

use crate::{config::Config, http::routes};

pub async fn run(config: Config) {
    let addr = format!("{}:{}", config.http.host, config.http.port);

    tracing::info!("Run server at http://{}", addr);

    let router = Router::new().merge(routes::get());

    axum::Server::bind(&addr.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}
