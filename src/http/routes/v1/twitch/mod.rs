use axum::Router;

mod user;

pub fn routes() -> Router {
    Router::new().nest("/user/:login", user::routes())
}
