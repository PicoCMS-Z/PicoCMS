use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Users(pub Vec<User>);

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}