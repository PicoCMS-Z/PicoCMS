use axum::{extract::State, Router};
// use mongodb::bson::Document;

mod database;
mod router;

mod model;
// use crate::database::Db;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .nest("/api", router::api::api_routes())
        .with_state(database::Db::new().await);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
