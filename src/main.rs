use axum::{
    routing::get,
    Router,
    response::Json
};
use serde_json::{Value, json};

#[tokio::main]


async fn main() {
    let app = Router::new().route("/", get(Json(json!({"id": "1"}))));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.expect("Server listener failed!");

    axum::serve(listener, app).await.unwrap();
}


