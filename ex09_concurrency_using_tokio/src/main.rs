mod api;

// The #[tokio::main] macro sets up the multi-threaded runtime
#[tokio::main]
async fn main() {
    println!("====== Example 09: Concurrency using Tokio ======");

    let mut handles = vec![];
    println!(
        "Number of worker threads: {}",
        tokio::runtime::Handle::current().metrics().num_workers()
    );
    println!("Launching 5 concurrent requests on the M4 cores...");

    for id in 1..=5 {
        // tokio::spawn moves the task to a background thread
        let handle = tokio::spawn(async move { api::fetch_post(id).await });
        handles.push(handle);
    }

    // Now we wait for all of them to finish
    for handle in handles {
        match handle.await.unwrap() {
            Ok(post) => println!("(Post #{}): Title: {}", post.id, post.title),
            Err(e) => eprintln!("Error: {e}"),
        }
    }

    println!("All requests completed!");
}
