use axum::routing::get;
use axum::Router;

use crate::database::Db;

use crate::api;

pub fn api_routes() -> Router<Db> {
    Router::new() 
}
