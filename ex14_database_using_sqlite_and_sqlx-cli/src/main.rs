mod api;

use api::{AppError, Post, Summary};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

// Set up a "Subscriber" at the very beginning of the `main` function. This is the part that
// actually decides where the logs go (usually your terminal).
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

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
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // 1. Initialize the logger
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("====== Example 14: Database using SQLite and SQLx-CLI ======");

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
        .with_state(shared_state)
        // 2. Add the TraceLayer to your router
        // This wraps every route in a logger
        // `tracing` keeps track of which logs belong to which request (and which user)
        // `tracing` is designed to be extremely low-overhead.
        // Auto logs: 1) When a request starts, 2) What the URL was, 3) How long it tool to finish,
        // and 4) The final status code
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Axum server starting on http://{}", addr);

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
            tracing::info!("Serving: {}", post.summarize());
            Ok(Json(post))
        }
        Err(e) => {
            // tracing::error! makes this stand out in your logs
            tracing::error!(error.details = ?e, "Failed to fetch post");
            Err(e)
        }
    }
}
