use axum::extract::State;
use axum::{extract, response};
// cursorのイテレータを使うため
use mongodb::bson::{self, Document};

use crate::database::Db;
use crate::model::user::*;

// TODO:error handling

pub async fn create(
    State(db): State<Db>,
    extract::Json(nwe_user): extract::Json<NewUser>,
) -> response::Json<User> {
    let user_collection = db.0.collection::<Document>("user");

    let user = User {
        id: bson::oid::ObjectId::new(),
        name: nwe_user.name,
        email: nwe_user.email,
        password: nwe_user.password,
    };

    let serialized_user = bson::to_bson(&user).unwrap();
    let serialized_user = serialized_user.as_document().unwrap();

    user_collection
        .insert_one(serialized_user.clone(), None)
        .await
        .unwrap();

    response::Json(user)
}

