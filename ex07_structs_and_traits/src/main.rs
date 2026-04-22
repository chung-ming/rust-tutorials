mod api;

use api::{Post, Summary};

fn main() {
    println!("====== Example 07: Structs and Traits ======");

    // Using the constructor
    let my_post = Post::new(
        1,
        String::from("Rust on M4"),
        String::from("Testing the M4 performance."),
    );

    // Using the trait method
    println!("{}", my_post.summarize());
}
