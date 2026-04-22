use anyhow::Context; // Provides a "Context" trait that lets you add info to errors
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

// We derive Serialize so Axum can send it to a browser and Deserialize so we can read it from an
// external API.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

// A simple Trait to demonstrate shared behavior
pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Post {
    fn summarize(&self) -> String {
        let short_title: String = self.title.chars().take(20).collect();
        let short_body: String = self.body.chars().take(20).collect();

        format!(
            "Post #{}: Title: {}..., Body: {}...",
            self.id, short_title, short_body
        )
    }
}

// This is the "Professional" way because:
// 1. Clarity: Instead of a Generic Error, there is a clear list of everything that can go wrong.
// 2. Automatic Conversion: Because we used `#[from] anyhow::Error`, Rust auto converts an
// unexpected error into `ApiError::Unexpected` when you use the `?` operator.
// 3. Performance: Unlike exceptions in other languages, these are just Enums. There is no
// stack-trace generation unless you explicitly ask for it, keeping the device's execution path
// lightning-fast.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Post with ID {0} was not found")]
    NotFound(u32),

    #[error("Invalid input: {0}")]
    BadRequest(String),

    #[error("External API error: {0}")]
    ExternalApi(String),

    // The 'transparent' attribute forwards the error message from anyhow
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

// Tell Axum how to turn AppError into an HTTP Response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(id) => (StatusCode::NOT_FOUND, format!("Post {} not found", id)),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::ExternalApi(msg) => (StatusCode::BAD_GATEWAY, msg),
            // We don't want to leak internal system details to users, so we give a generic message
            AppError::Unexpected(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred".to_string(),
            ),
        };

        // Create a JSON body for the error
        let body = Json(json!({"error": error_message, "status": status.as_u16()}));

        (status, body).into_response()
    }
}

// An asynchronous function to fetch a post from an external source
pub async fn fetch_external_post(client: &reqwest::Client, id: u32) -> Result<Post, AppError> {
    let url = format!("https://jsonplaceholder.typicode.com/posts/{id}");

    // Get JSON and auto parse it into our Post struct
    let response = client
        .get(&url)
        // .send() returns a future that performs the HTTP request, and then you
        // .await the future
        .send()
        .await
        // Use .context("string") for static messages
        .context("Attempting to reach the external JSON Placeholder API")
        .map_err(AppError::Unexpected)?;

    // Explicitly handle the 404
    if response.status() == 404 {
        return Err(AppError::NotFound(id));
    }

    // Ensure the status is successful (200 OK) before parsing
    if !response.status().is_success() {
        return Err(AppError::ExternalApi(format!(
            "External server returned status: {}",
            response.status()
        )));
    }

    // Parse the JSON
    let post = response
        .json::<Post>()
        .await
        // Use .with_context(|| format!(...)) for dynamic messages. The closure (||) ensures the
        // string formatting only happens if an error actually occurs, saving your M4 CPU cycles
        // during the "happy path."
        .with_context(|| format!("Failed to parse JSON for post ID: {}", id))
        .map_err(AppError::Unexpected)?;

    Ok(post)
}
