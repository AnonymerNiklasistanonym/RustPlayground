//! Hint: main.rs is not included in the generated API documentation!

/// Hint: anything in main.rs is usually not included in the generated API documentation!
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Single line comment

/*
 * Multiline comment
 */

fn main() {
    #[cfg(debug_assertions)]
    println!("debug");
    #[cfg(not(debug_assertions))]
    println!("release");

    println!("{}", add(2, 3));
}
