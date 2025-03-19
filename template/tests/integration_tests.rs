//! Integration tests for {{project-name}}
//!
//! Unlike unit tests, these tests check the public API
//! of the crate as a whole, as it would be used by other crates.

use {{ project-name | snake_case }}::calculate;

#[test]
fn test_arithmetic_operations() {
    // Test that our arithmetic functions work together correctly
    let a = 10;
    let b = 5;

    let sum = calculate(a, b, "add").unwrap();
    assert_eq!(sum, 15);

    let quotient = calculate(sum, b, "divide").unwrap();
    assert_eq!(quotient, 3);
}
