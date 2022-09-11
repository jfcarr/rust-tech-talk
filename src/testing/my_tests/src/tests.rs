// The use super::* gives access to all functions in the crate.
use super::*;

// Individual test functions are given a #[test] attribute.
#[test]
fn test_add() {
    assert_eq!(add(10, 5), 15);
}
