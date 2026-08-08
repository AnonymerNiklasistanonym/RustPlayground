fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    #[cfg(debug_assertions)]
    println!("debug");
    #[cfg(not(debug_assertions))]
    println!("release");

    println!("{}", add(2, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
