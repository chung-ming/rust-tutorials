use rand::Rng;

fn main() {
    println!("====== Example 05: HTTP Request using Reqwest ======");

    // Random Number Logic
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen_range(1..10);
    println!("Fetching data for ID: {id}...");

    // HTTP Request
    // We use a sample API that returns a JSON placeholder
    let url = format!("https://jsonplaceholder.typicode.com/posts/{id}");

    // HTTP Response
    // In Rust, operations that can fail return a 'Result' type
    // .unwrap() is a quick way to say "give me the value or crash if there's an error"
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("\n--- API Response ---\n{response}");
}
