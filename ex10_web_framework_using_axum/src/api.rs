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
        // Calculate the safe end point for the title and body
        let short_title: String = self.title.chars().take(20).collect();
        let short_body: String = self.body.chars().take(20).collect();

        format!(
            "Post #{}: Title: {}, Body: {}...",
            self.id, short_title, short_body
        )
    }
}
