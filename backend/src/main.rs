use axum::{extract::Path, response::IntoResponse, routing::get, Router};
// use uuid::Uuid;

mod generate_magic_code;

const IP_PORT: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {

    let wordlist = generate_magic_code::read_wordlist();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/{id}", get(return_id))
        .route("/code", get(generate_magic_code::generate_code(&wordlist)));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Running on: https://{IP_PORT}");
}

async fn return_id(Path(user_id): Path<u64>) -> impl IntoResponse {
    format!("You tried to get {user_id}")
}

// async fn new_uuid() -> impl IntoResponse {
//     Uuid::new_v4().to_string()
// }
