/// Returns the sum of two numbers.
///
/// # Examples
///
/// ```
/// use {{project-name}}::add;
///
/// assert_eq!(add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Divides the first number by the second.
///
/// # Examples
///
/// ```
/// use {{project-name}}::divide;
///
/// assert_eq!(divide(10, 2), 5);
/// ```
///
/// # Panics
///
/// Panics if the divisor is zero.
///
/// ```should_panic
/// use {{project-name}}::divide;
///
/// divide(10, 0); // This will panic
/// ```
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-2, 2), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), 5);
        assert_eq!(divide(0, 1), 0);
        assert_eq!(divide(-10, 2), -5);
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_divide_by_zero() {
        divide(10, 0);
    }
}
