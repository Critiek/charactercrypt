use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use uuid::Uuid;

const IP_PORT: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    println!("Running on: https://{}", IP_PORT);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/{id}", get(return_id))
        .route("/uuid", get(new_uuid));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn return_id(Path(user_id): Path<u64>) -> impl IntoResponse {
    format!("You tried to get {}", user_id)
}

async fn new_uuid() -> impl IntoResponse {
    Uuid::new_v4().to_string()
}
