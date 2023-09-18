use std::{sync::Arc, time::Duration};

use axum::{
    http::{HeaderValue, Method},
    middleware, Extension, Router,
};
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, ORIGIN},
    StatusCode,
};
use sqlx::{Pool, Postgres};
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer};

use crate::{config::Config, dao, error::AppError, jwt, service};

use super::{
    middleware::{logger_middleware, request_id_middleware},
    routes,
};

pub fn new(config: Config, pool: Arc<Box<Pool<Postgres>>>) -> Router {
    let jwt = jwt::Jwt::new(&config.jwt.secret);

    let user_dao = dao::UserDao::new(Arc::clone(&pool));
    let twitch_data_dao = dao::TwitchDataDao::new(Arc::clone(&pool));
    let token_dao = dao::TokenDao::new(Arc::clone(&pool));

    let auth_service =
        service::AuthService::new(config.twitch, jwt, user_dao, twitch_data_dao, token_dao);

    Router::new()
        .merge(routes::get().fallback(handler_404))
        .layer(Extension(Arc::new(auth_service)))
        .layer(middleware::from_fn(logger_middleware))
        .layer(middleware::from_fn(request_id_middleware))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers([AUTHORIZATION, ACCEPT, ORIGIN])
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap()),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
}

async fn handler_404() -> AppError {
    AppError::new(StatusCode::NOT_FOUND).message("not found".to_string())
}
