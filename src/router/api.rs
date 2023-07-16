use axum::routing::{get,post};

use axum::Router;

use crate::database::Db;

use crate::api;

pub fn api_routes() -> Router<Db> {
    let user_ruter=    Router::new() 
        .route("/create", post(api::user::create))
        .route("/read", get(api::user::read_all))
        .route("/read/:id", get(api::user::read_one))
        .route("/update/:id", post(api::user::update))
        .route("/delete/:id", post(api::user::delete));

    Router::new()
        .nest("/v0-alpha/user", user_ruter)
}
