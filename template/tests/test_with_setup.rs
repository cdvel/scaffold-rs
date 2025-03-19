//! Tests that demonstrate setup/teardown patterns

mod common;

use {{ project-name | snake_case }}::calculate;
use common::setup;

#[test]
fn test_with_environment() {
    // Use the common setup function
    let test_env = setup();

    // Verify the environment is properly initialized
    assert!(test_env.initialized);
    assert!(test_env.run_test_operation());

    // Run the actual test
    assert_eq!(calculate(3, 4, "add").unwrap(), 7);

    // TestEnv::drop will be called automatically here
    // which will clean up the environment
}
