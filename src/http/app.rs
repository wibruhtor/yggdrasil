use std::{sync::Arc, time::Duration};

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN, USER_AGENT},
        HeaderValue, Method, StatusCode,
    },
    middleware, routing, Extension, Router,
};
use axum_prometheus::PrometheusMetricLayer;
use sqlx::{Pool, Postgres};
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer};

use crate::{config::Config, crypt, dao, error::AppError, jwt, service, webapi};

use super::{
    middleware::{logger_middleware, request_id_middleware},
    routes,
};

pub fn new(config: Config, pool: Arc<Box<Pool<Postgres>>>) -> Router {
    let jwt = jwt::Jwt::new(&config.jwt.secret);
    let crypt = crypt::Crypt::new(&config.crypt.secret);

    let user_dao = dao::UserDao::new(Arc::clone(&pool));
    let twitch_data_dao = dao::TwitchDataDao::new(Arc::clone(&pool));
    let token_dao = dao::TokenDao::new(Arc::clone(&pool), Arc::clone(&crypt));
    let ban_word_filter_dao = dao::BanWordFilterDao::new(Arc::clone(&pool));
    let ban_word_dao = dao::BanWordDao::new(Arc::clone(&pool));
    let chat_settings_dao = dao::ChatSettingsDao::new(Arc::clone(&pool));

    let twitch_web_api = webapi::TwitchWebApi::new(Arc::clone(&config.twitch));

    let auth_service = service::AuthService::new(
        Arc::clone(&config.twitch),
        Arc::clone(&jwt),
        Arc::clone(&user_dao),
        Arc::clone(&twitch_data_dao),
        Arc::clone(&token_dao),
    );
    let session_service = service::SessionService::new(Arc::clone(&token_dao));
    let ban_word_service =
        service::BanWordService::new(Arc::clone(&ban_word_filter_dao), Arc::clone(&ban_word_dao));
    let chat_service = service::ChatService::new(Arc::clone(&chat_settings_dao));
    let twitch_service = service::TwitchService::new(Arc::clone(&twitch_web_api));

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    Router::new()
        .route(
            "/metrics",
            routing::get(|| async move { metric_handle.render() }),
        )
        .merge(routes::get().fallback(handler_404))
        .layer(Extension(Arc::new(auth_service)))
        .layer(Extension(Arc::new(session_service)))
        .layer(Extension(Arc::new(ban_word_service)))
        .layer(Extension(Arc::new(chat_service)))
        .layer(Extension(Arc::new(twitch_service)))
        .layer(middleware::from_fn(logger_middleware))
        .layer(prometheus_layer)
        .layer(middleware::from_fn(request_id_middleware))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers([AUTHORIZATION, ACCEPT, ORIGIN, CONTENT_TYPE, USER_AGENT])
                .allow_origin(config.http.allow_origin.parse::<HeaderValue>().unwrap()),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
}

async fn handler_404() -> AppError {
    AppError::new(StatusCode::NOT_FOUND).message("not found".to_string())
}
