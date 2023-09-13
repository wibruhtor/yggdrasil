use crate::{config::Config, http::router};

pub async fn run(config: Config) {
    let addr = format!("{}:{}", config.http.host, config.http.port);

    println!("Run server on http://{}", addr);

    let router = router::new();

    axum::Server::bind(&addr.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}
