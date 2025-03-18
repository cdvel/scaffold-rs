//! # {{ project-name | snake_case }}
//!
//! `{{ project-name | snake_case }}` is a library that {{ project-description }}.

// Re-export core modules
pub mod app;
pub mod config;
pub mod error;
pub mod logging;

// Feature-gated modules
#[cfg(feature = "async")]
pub mod async_utils;

#[cfg(feature = "chrono")]
pub mod time_utils;

/// A simple example function that demonstrates error handling.
///
/// # Examples
///
/// ```
/// use {{ project-name | snake_case }}::calculate;
///
/// let result = calculate(10, 2, "divide").unwrap();
/// assert_eq!(result, 5);
/// ```
pub fn calculate(a: i32, b: i32, operation: &str) -> error::Result<i32> {
    match operation {
        "add" => Ok(a + b),
        "subtract" => Ok(a - b),
        "multiply" => Ok(a * b),
        "divide" => {
            if b == 0 {
                Err(error::Error::InvalidInput("Cannot divide by zero".into()))
            } else {
                Ok(a / b)
            }
        }
        _ => Err(error::Error::InvalidOperation(format!(
            "Unknown operation: {}",
            operation
        ))),
    }
}

// Convenient re-exports
pub use app::App;
#[cfg(feature = "chrono")]
pub use time_utils::{current_timestamp, parse_timestamp};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_valid_operations() {
        assert_eq!(calculate(5, 3, "add").unwrap(), 8);
        assert_eq!(calculate(5, 3, "subtract").unwrap(), 2);
        assert_eq!(calculate(5, 3, "multiply").unwrap(), 15);
        assert_eq!(calculate(6, 3, "divide").unwrap(), 2);
    }

    #[test]
    fn test_calculate_division_by_zero() {
        let result = calculate(5, 0, "divide");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), error::Error::InvalidInput(_)));
    }

    #[test]
    fn test_calculate_invalid_operation() {
        let result = calculate(5, 3, "power");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            error::Error::InvalidOperation(_)
        ));
    }
}
