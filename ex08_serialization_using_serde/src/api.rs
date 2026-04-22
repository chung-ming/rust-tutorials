use serde::Deserialize; // Imports the Deserialize trait

// This macro generates the code to map JSON to this struct
#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

// Returns a Result
// On success: Post
// On failure: A dynamic Boxed error (can hold any error type)
pub fn fetch_post(id: u32) -> Result<Post, Box<dyn std::error::Error>> {
    let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);

    // Instead of .text(), we use .json::Post<>()
    // Rust infers that it should turn the JSON into a Post struct!
    let response = reqwest::blocking::get(url)?.json::<Post>()?;

    Ok(response)
}
