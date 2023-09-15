use axum::Router;

mod auth;
mod v1;

pub fn get() -> Router {
    Router::new()
        .nest("/v1", v1::routes())
        .nest("/auth", auth::routes())
}
