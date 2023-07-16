use axum::extract::State;
use axum::response::Json;
use axum::routing::get;
use axum::Router;
use mongodb::bson;
use mongodb::bson::Document;

use crate::database::Db;



pub fn api_routes() -> Router<Db> {
    Router::new() 
}
