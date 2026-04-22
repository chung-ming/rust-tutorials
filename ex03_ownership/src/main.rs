mod sample1;
mod sample2;

fn main() {
    println!("====== Example 03: Ownership ======");
    sample1::main();
    sample2::main();
}

// The 3 Rules of Ownership:
// Rule 1: Each value in Rust has a variable that's called its owner.
// Rule 2: There can only be one owner at a time.
// Rule 3: When the owner goes out of scope, the value will be dropped.
// Because there is no Garbage Collector running in the background, the CPU cycles are spent
// entirely on the code, not on cleaning up memory.

// The "Golden Rule" of Borrowing:
// You can have either one mutable reference (&mut T) OR any number of immutable references (&T) at
// the same time. You can't have both. This prevents "Data Races" where two parts of your code try
// to change the same memory at once.
