use axum::routing::get;
use axum::Router;

pub fn api_routes() -> Router {
    Router::new().route("/", get(hello_world))
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
