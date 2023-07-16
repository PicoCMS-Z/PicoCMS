use axum::extract::State;
use axum::response::Json;
use axum::routing::get;
use axum::Router;
use mongodb::bson;
use mongodb::bson::Document;

use crate::database::Db;

use super::super::model::user::User;

pub fn api_routes() -> Router<Db> {
    Router::new()
        .route("/", get(hello_world))
        .route("/user", get(get_user))
}

// sample handler
async fn hello_world() -> &'static str {
    "Hello, World!"
}


//sample detabase handler
async fn get_user(State(db): State<Db>) -> Json<User> {
    let user_collection = db.0.collection::<Document>("user");
    let user = user_collection.find_one(None, None).await.unwrap().unwrap();
    let user: User = bson::from_bson(bson::Bson::Document(user)).unwrap();
    Json(user)
}
