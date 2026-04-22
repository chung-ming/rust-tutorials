mod api;

use api::{AppError, Post, Summary};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
// 1. Updated State to hold the DB Pool
struct AppState {
    db: SqlitePool,
    visitor_count: Mutex<u32>,
    http_client: reqwest::Client,
}

// Transform the main.rs into a web server listener.
// Listens for an "input" (HTTP Request) and returns an "output" (HTTP Response).
#[tokio::main]
async fn main() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Initialize the logger
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("====== Example 14: Database using SQLite and SQLx-CLI ======");

    // 2. Load the .env file into the process environment
    // .ok() ignores the error if the file is missing
    dotenvy::dotenv().ok();

    // 3. Connect to the SQLite DB defined in your .env
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to SQLite");

    // Build a client with a 35-second timeout
    let http_client = reqwest::Client::builder()
        // 5-secs for the connection (TCP handshake)
        .connect_timeout(Duration::from_secs(5))
        // 30-secs for the connection + wait time for the response body
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client");

    // Wrap the state in an Arc so it can be shared across cores
    let shared_state = Arc::new(AppState {
        db: pool,
        visitor_count: Mutex::new(0),
        http_client,
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health_check))
        // New route: Fetch a real post by ID from the internet
        .route("/post/:id", get(get_post))
        // Share the state with all routes
        .with_state(shared_state)
        // Add the TraceLayer to your router
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
    tracing::info!("Visitor #{} has arrived!", count);
    format!("You are visitor number {}!", count)
}

async fn health_check() -> &'static str {
    "OK"
}

// Connects to api.rs and uses the 'id' from the URL
// Returns the custom AppError
async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u32>,
) -> Result<Json<Post>, AppError> {
    if id == 0 {
        tracing::warn!("Rejecting request for invalid ID: 0");
        return Err(AppError::BadRequest(format!(
            "ID {} is not a positive integer",
            id
        )));
    }

    // 1. Check SQLite first
    let cached_post = sqlx::query_as!(
        Post,
        "SELECT id as \"id: u32\", title, body FROM posts WHERE id = ?",
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

    if let Some(post) = cached_post {
        tracing::info!("Cache Hit for post {}", id);
        return Ok(Json(post));
    }

    // 2. Fetch from API if not found
    tracing::warn!("Cache Miss for post {}. Fetching from external API...", id);
    let post = match api::fetch_external_post(&state.http_client, id).await {
        Ok(post) => {
            tracing::info!("Serving: {}", post.summarize());
            post // Yield the post object to the variable post
        }
        Err(e) => {
            // tracing::error! makes this stand out in your logs
            tracing::error!(error.details = ?e, "Failed to fetch post");
            // Exit the function early because we can't proceed without a post
            return Err(e);
        }
    };

    // 3. Persist to SQLite for future visitors
    sqlx::query!(
        "INSERT INTO posts (id, title, body) VALUES (?, ?, ?)",
        post.id,
        post.title,
        post.body
    )
    .execute(&state.db)
    .await
    .map_err(|e| anyhow::anyhow!("Failed to persist post: {}", e))?;

    // 4. Final return
    Ok(Json(post))
}
