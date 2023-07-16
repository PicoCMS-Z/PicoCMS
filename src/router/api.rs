use axum::routing::{get,post};
use axum::Router;

use crate::database::Db;

use crate::router::api;

pub fn api_routes() -> Router<Db> {
    Router::new() 
        .route("/read", get(api::user::read_all))
        .route("/read/:id", get(api::user::read_one))
        .route("/create", post(api::user::create))
}
