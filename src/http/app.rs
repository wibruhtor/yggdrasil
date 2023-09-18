use std::sync::Arc;

use axum::{middleware, Extension, Router};
use sqlx::{Pool, Postgres};

use crate::{config::Config, dao, jwt, service};

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

    // let app_state = AppState { auth_service };

    Router::new()
        .merge(routes::get())
        .layer(Extension(Arc::new(auth_service)))
        .layer(middleware::from_fn(logger_middleware))
        .layer(middleware::from_fn(request_id_middleware))
}
