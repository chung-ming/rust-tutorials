mod api;

use api::{Post, Summary};
use axum::{Json, Router, routing::get};
use std::net::SocketAddr;

// Transform the main.rs into a web server listener.
// Listens for an "input" (HTTP Request) and returns an "output" (HTTP Response).
#[tokio::main]
async fn main() {
    println!("====== Example 10: Web Framework using Axum ======");

    // 1. Build our "Router" - maps URLs to functions
    // Define a root route, a health check route, and a dynamic post route
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health_check))
        .route("/post", get(get_sample_post));

    // 2. Define the address (Localhost on port 3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Axum server starting on http://{}", addr);

    // 3. Start the server using Tokio's TCP listener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// A root handler
async fn handler() -> &'static str {
    "Hello, you are running an Axum server!"
}

// A health check handler
async fn health_check() -> &'static str {
    "OK"
}

// Json handler that returns our Post struct
// In Axum, we use Extractors to parse `request` objects.
// If you want JSON, you put `Json<MyStruct>` in the function arguments.
// If you want a URL ID, you put `Path<u32>`.
// If the data isn't there, it returns an error to the user before the function even runs. This is
// Type Safety for the Web.
async fn get_sample_post() -> Json<Post> {
    let post = Post {
        id: 42,
        title: String::from("Learning Axum"),
        body: String::from("The M4 silicon handles this with zero effort."),
    };

    // Print a summary to the console using our Trait
    println!("Serving: {}", post.summarize());

    // Axum auto converts this into JSON because of Serde
    Json(post)
}
