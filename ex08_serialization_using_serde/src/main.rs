mod api;

use rand::Rng;

fn main() {
    println!("====== Example 08: Serialization using Serde ======");

    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen_range(1..10);

    println!("Attempting to fetch post {id}...");

    match api::fetch_post(id) {
        Ok(post) => {
            println!("Successfully deserialized post!");
            println!("Post Object: {:?}", post);
            println!("ID: {}", post.id);
            println!("Title: {}", post.title);
            println!("Body: {}...", &post.body[..20]);
        }
        Err(e) => eprintln!("Failed to fetch data. Error: {e}"),
    }
}
