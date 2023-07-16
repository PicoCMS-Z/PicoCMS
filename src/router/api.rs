use axum::routing::get;
use axum::Router;

use crate::database::Db;



pub fn api_routes() -> Router<Db> {
    Router::new() 
        .route("/create", post(api::user::create))
}
