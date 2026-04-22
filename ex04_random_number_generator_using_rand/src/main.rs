use rand::Rng;

fn main() {
    println!("====== Example 04: Random Number Generator using Rand ======");

    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..101); // Generates a number between 1 and 100

    println!("Your lucky random number is {n}");
}
