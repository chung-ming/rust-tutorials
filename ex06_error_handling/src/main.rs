mod api;

use rand::Rng;

fn main() {
    println!("====== Example 06: Error Handling ======");

    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen_range(1..10);

    println!("Attempting to fetch post {id}...");

    // We call our function and "match" on the result
    match api::fetch_post(id) {
        Ok(content) => {
            println!("Success! Data received:");
            println!("{content}");
        }
        Err(e) => {
            // Instead of crashing, we print a friendly error message
            eprintln!("Failed to fetch data. Error: {e}");
            eprintln!("Check your internet connection and try again.");
        }
    }
}
