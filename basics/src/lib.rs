//! Documentation comment: Generate docs for the item enclosed by this (the crate)
//!
//! ```sh
//! cargo doc && python -m http.server -d target/doc
//! ```

/// Documentation comment: Generates docs for whatever follows it (the add function)
///
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = example_project::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
