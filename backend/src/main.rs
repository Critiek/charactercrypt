use axum::{extract::Path, response::IntoResponse, routing::get, routing::post, Router};
use charactercrypt::AppState;
use http::{header, Method, Request, Response};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
mod magic_code;

const IP_PORT: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let wordlist = magic_code::read_wordlist();

    let app_state = AppState {
        wordlist: Arc::new(wordlist),
    };

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/{id}", get(return_id))
        .route("/code", get(magic_code::generate_code))
        .route("/submit_code", post(magic_code::handle_code_submissions))
        .with_state(app_state)
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running on: https://{IP_PORT}");
    axum::serve(listener, app).await.unwrap();
}

async fn return_id(Path(user_id): Path<u64>) -> impl IntoResponse {
    format!("You tried to get {user_id}")
}

// async fn new_uuid() -> impl IntoResponse {
//     Uuid::new_v4().to_string()
// }
