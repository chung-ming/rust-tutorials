use reqwest;

// Returns a Result
// On success: String
// On failure: A dynamic Boxed error (can hold any error type)
pub fn fetch_post(id: u32) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);

    // The '?' operator is magic.
    // It means: "If this succeeds, give me the value. If it fails, return the error early."
    let response = reqwest::blocking::get(url)?.text()?;

    Ok(response)
}
