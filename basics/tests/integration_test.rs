use example_project::add;

#[test]
fn adds_two_numbers_integration() {
    assert_eq!(add(2, 3), 5);
}
