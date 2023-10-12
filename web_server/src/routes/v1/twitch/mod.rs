use axum::Router;

mod badges;
mod emotes;
mod user;

pub fn routes() -> Router {
    Router::new()
        .nest("/badges", badges::routes())
        .nest("/emotes", emotes::routes())
        .nest("/user/:login", user::routes())
}
