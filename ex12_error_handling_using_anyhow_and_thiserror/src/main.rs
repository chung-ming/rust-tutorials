mod api;

use api::{AppError, Post, Summary};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

// As the app grows, using `String` for errors like `Err(format!(...))` becomes a nightmare. You
// can't easily tell a "Network Error" apart from a "Parsing Error".
// To fix this, we use two industry-standard creates:
// 1. thiserror: Used in `api.rs` to define specific, named errors.
// 2. anyhow: Used in `main.rs` to group different errors together easily.

// In Axum, a handler can return anything that implements the `IntoResponse` trait. By implementing
// this for our own error type, we can control exactly what HTTP status code and message the user gets.

// Create a struct to hold our "Global State"
struct AppState {
    visitor_count: Mutex<u32>,
}

// Transform the main.rs into a web server listener.
// Listens for an "input" (HTTP Request) and returns an "output" (HTTP Response).
#[tokio::main]
async fn main() {
    println!("====== Example 12: Error Handling using Anyhow and Thiserror ======");

    // Wrap the state in an Arc so it can be shared across cores
    let shared_state = Arc::new(AppState {
        visitor_count: Mutex::new(0),
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health_check))
        // New route: Fetch a real post by ID from the internet
        .route("/post/:id", get(get_post))
        // Share the state with all routes
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Axum server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(state): State<Arc<AppState>>) -> String {
    // Lock the mutex to update the counter
    let mut count = state.visitor_count.lock().unwrap();
    *count += 1;
    format!("You are visitor number {}!", count)
}

async fn health_check() -> &'static str {
    "OK"
}

// Connects to api.rs and uses the 'id' from the URL
// Returns the custom AppError
async fn get_post(Path(id): Path<u32>) -> Result<Json<Post>, AppError> {
    match api::fetch_external_post(id).await {
        Ok(post) => {
            println!("Serving: {}", post.summarize());
            Ok(Json(post))
        }
        Err(e) => {
            // Use {:?} to see the anyhow context's full error chain
            // Use {} to only see the top-level error
            eprintln!("DEBUG LOG: {:?}", e);
            Err(e)
        }
    }
}
