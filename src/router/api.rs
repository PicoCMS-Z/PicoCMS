use axum::routing::get;
use axum::Router;

use crate::database::Db;

use crate::api;

pub fn api_routes() -> Router<Db> {
    Router::new() 
        .route("/create", post(api::user::create))
        .route("/read", get(api::user::read_all))
}
