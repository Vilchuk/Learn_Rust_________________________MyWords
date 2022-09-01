mod app_config;

use axum::{body::Body, response::Json, routing::get, Router};
use serde_json::{json, Value};
use std::net::SocketAddr;

use api::db;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(root));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    println!("The server is working on http://localhost:{}", port);
    // run it with hyper on localhost:PORT
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<Value> {
    let users = db::users::get_users().await.unwrap();
    Json(json!({ "data": json!(users) }))
}
