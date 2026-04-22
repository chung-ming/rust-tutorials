pub fn main() {
    println!("====== Sample 01 ======");

    // String is stored on the heap
    let s1 = String::from("hello");
    let s2 = s1; // Ownership is MOVED from s1 to s2

    // println!("{s1}"); // UNCOMMENT THIS: It will throw an error!
    println!("{s2}");
}
