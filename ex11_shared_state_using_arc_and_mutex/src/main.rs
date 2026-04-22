mod api;

use api::{Post, Summary};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

// In a multi- threaded server like Axum, you can't just use a global variable. If two people hit
// your server at the exact same time on two different CPU cores, they might both try to update a
// counter at once, causing a Data Race.
// To fix this, we use two tools:
// 1. Mutex (Mutual Exclusion): A "lock" that only allows one thread at a time to touch the data.
// 2. Arc (Atomic Reference Counter): A "smart pointer" that allows multiple threads to safely own
// the same Mutex.
// Because of the Arc, every core on the chip can safely access `visitor_count`. The Mutex ensures
// that even if 100 people refresh your page at the same nanosecond, the counter will always be
// accurate. Rust's compiler won't let you compile if you forget the Mutex.

// 1. Create a struct to hold our "Global State"
struct AppState {
    visitor_count: Mutex<u32>,
}

// Transform the main.rs into a web server listener.
// Listens for an "input" (HTTP Request) and returns an "output" (HTTP Response).
#[tokio::main]
async fn main() {
    println!("====== Example 11: Shared State using Arc and Mutex ======");

    // 2. Wrap the state in an Arc so it can be shared across cores
    let shared_state = Arc::new(AppState {
        visitor_count: Mutex::new(0),
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health_check))
        // 3. New route: Fetch a real post by ID from the internet
        .route("/post/:id", get(get_post))
        // Share the state with all routes
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Axum server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(state): State<Arc<AppState>>) -> String {
    // 4. Lock the mutex to update the counter
    let mut count = state.visitor_count.lock().unwrap();
    *count += 1;
    format!("You are visitor number {}!", count)
}

async fn health_check() -> &'static str {
    "OK"
}

// 5. Connects to api.rs and uses the 'id' from the URL
// Path(id): An extractor that looks at `/posts/5` and pulls out the `5` as a `u32`.
// api::fetch_external_post(id).await: To get real data from the web.
async fn get_post(Path(id): Path<u32>) -> Result<Json<Post>, String> {
    match api::fetch_external_post(id).await {
        Ok(post) => {
            println!("Serving: {}", post.summarize());
            Ok(Json(post))
        }
        Err(e) => Err(format!("Failed to fetch post: {}", e)),
    }
}
