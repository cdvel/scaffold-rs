//! Common utilities for tests
//!
//! This module contains helper functions and setup/teardown code
//! that is shared across multiple test files.

/// A test helper that creates a test environment
pub fn setup() -> TestEnv {
    println!("Setting up test environment");
    TestEnv { initialized: true }
}

/// A simple struct representing the test environment
pub struct TestEnv {
    pub initialized: bool,
}

impl TestEnv {
    /// Perform some test operation
    pub fn run_test_operation(&self) -> bool {
        self.initialized
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        println!("Cleaning up test environment");
    }
}
