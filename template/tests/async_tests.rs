#[cfg(feature = "async")]
mod async_tests {
    use {{ project-name | snake_case }}::async_utils::{delayed_timestamp, fetch_data};

    #[tokio::test]
    async fn test_delayed_timestamp() {
        let result = delayed_timestamp(10).await;
        assert!(result.contains("Current time (after 10ms delay)"));
    }

    #[tokio::test]
    async fn test_fetch_data() {
        let data = fetch_data("test-resource").await.unwrap();
        assert_eq!(data, "Data for resource test-resource");
    }
}
