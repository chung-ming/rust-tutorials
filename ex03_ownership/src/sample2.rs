pub fn main() {
    println!("====== Sample 02 ======");

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // We pass a reference, not the value

    println!("The length of '{s1}' is {len}."); // s1 is still valid here!
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
}
