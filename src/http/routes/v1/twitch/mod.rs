use axum::Router;

mod badges;
mod emotes;
mod user;

pub fn routes() -> Router {
    Router::new()
        .nest("/user/:login", user::routes())
        .nest("/emotes", emotes::routes())
        .nest("/badges", badges::routes())
}
