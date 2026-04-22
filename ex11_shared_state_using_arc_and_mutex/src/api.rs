use serde::{Deserialize, Serialize};

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

// An asynchronous function to fetch a post from an external source
pub async fn fetch_external_post(
    id: u32,
) -> Result<Post, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://jsonplaceholder.typicode.com/posts/{id}");

    // Using reqwest to get JSON and auto parse it into our Post struct
    let response = reqwest::get(url).await?.json::<Post>().await?;

    Ok(response)
}
