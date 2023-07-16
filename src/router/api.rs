use axum::routing::get;

use axum::Router;

use crate::database::Db;

use crate::api;

pub fn api_routes() -> Router<Db> {
    Router::new().nest("/v0-alpha/users", user_router())
}

fn user_router() -> Router<Db> {
    Router::new()
        .route("/", get(api::user::read_all).post(api::user::create))
        .route(
            "/:id",
            get(api::user::read_one)
                .put(api::user::update)
                .delete(api::user::delete),
        )
}
