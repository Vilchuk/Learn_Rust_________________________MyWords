mod app_config;
use api::db;
use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(users))
        .route("/users/:id", get(user));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let address = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on {}", address);

    // run it with hyper on localhost:PORT
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<Value> {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    Json(json!({
        "endpoints": [
            {
                "method": "GET",
                "path": format!("http://localhost:{}/users", port),
                "description": "Returns all users"
            },
            {
                "method": "GET",
                "path": format!("http://localhost:{}/users/1", port),
                "description": "Returns user by id"
            }
        ]
    }))
}

async fn users() -> Json<Value> {
    let users = match db::users::get_users().await {
        Ok(users) => users,
        Err(e) => {
            println!("Error: {}", e);
            return Json(json!({ "error": e.to_string() }));
        }
    };

    Json(json!({ "users": users }))
}

async fn user(id: String) -> Json<Value> {
    println!("id: {}", id);
    let id = match id.parse::<i64>() {
        Ok(id) => id,
        Err(e) => {
            println!("Error: {}", e);
            return Json(json!({ "error": e.to_string() }));
        }
    };

    let user = match db::users::get_user_by_id(id).await {
        Ok(user) => user,
        Err(e) => {
            println!("Error: {}", e);
            return Json(json!({ "error": e.to_string() }));
        }
    };

    Json(json!({ "user": user }))
}
