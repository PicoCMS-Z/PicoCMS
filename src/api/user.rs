use axum::extract::State;
use axum::{extract, response};
// cursorのイテレータを使うため
use futures::StreamExt;

use crate::database::Db;
use mongodb::bson::{self, Document};

use crate::model::user::*;
pub async fn read_all(State(db): State<Db>) -> response::Json<Users> {
    let user_collection = db.0.collection("user");

    let cursor = user_collection.find(None, None).await.unwrap();

    let users = cursor
        .map(|result| {
            let doc = result.unwrap();
            bson::from_bson(bson::Bson::Document(doc)).unwrap()
        })
        .collect()
        .await;

    response::Json(Users(users))
}

