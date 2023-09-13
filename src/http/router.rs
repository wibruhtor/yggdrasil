use crate::config::Config;
use axum::{routing::get, Router};

pub async fn run(config: Config) {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let addr = format!("{}:{}", config.http.host, config.http.port);

    println!("Run server on http://{}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
