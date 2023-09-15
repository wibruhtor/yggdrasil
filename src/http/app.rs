use std::sync::Arc;

use axum::{Extension, Router};
use sqlx::{Pool, Postgres};

use crate::{
    config::Config,
    dao,
    service::{self, auth::AuthService},
};

use super::routes;

pub struct AppState {
    pub auth_service: AuthService,
}

pub fn new(config: Config, pool: Arc<Box<Pool<Postgres>>>) -> Router {
    let user_dao = dao::user::UserDao::new(Arc::clone(&pool));

    let auth_service = service::auth::AuthService::new(config.twitch, user_dao);

    let app_state = AppState { auth_service };

    Router::new()
        .merge(routes::get())
        .layer(Extension(Arc::new(app_state)))
}
