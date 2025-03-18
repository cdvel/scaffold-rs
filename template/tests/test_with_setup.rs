//! Tests that demonstrate setup/teardown patterns

mod common;

use {{project-name}}::add;
use common::setup;

#[test]
fn test_with_environment() {
    // Use the common setup function
    let test_env = setup();

    // Verify the environment is properly initialized
    assert!(test_env.initialized);
    assert!(test_env.run_test_operation());

    // Run the actual test
    assert_eq!(add(3, 4), 7);

    // TestEnv::drop will be called automatically here
    // which will clean up the environment
}
