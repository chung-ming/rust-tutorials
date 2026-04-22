use reqwest;
use serde::Deserialize;

// Added Clone to make copying easier
#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    pub id: u32,
    pub title: String,
}

// Added 'async' keyword
// The function returns a Future, i.e. a promise that a value will exist later
pub async fn fetch_post(id: u32) -> Result<Post, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://jsonplaceholder.typicode.com/posts/{id}");

    // We now use .await after each asynchronous step
    let response = reqwest::get(url).await?.json::<Post>().await?;

    Ok(response)
}
