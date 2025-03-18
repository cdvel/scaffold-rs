//! Async utilities and examples
//!
//! This module is only compiled when the `async` feature is enabled.

#[cfg(feature = "async")]
use tokio::time::{sleep, Duration};
#[cfg(feature = "async")]
use chrono::{Utc, DateTime};

/// Example async function that demonstrates both tokio and chrono usage
#[cfg(feature = "async")]
pub async fn delayed_timestamp(delay_ms: u64) -> String {
    // Use tokio sleep for async delay
    sleep(Duration::from_millis(delay_ms)).await;

    // Use chrono crate for timestamp
    let now: DateTime<Utc> = Utc::now();
    format!("Current time (after {}ms delay): {}",
        delay_ms,
        now.to_rfc3339()
    )
}

/// Simple async operation example
#[cfg(feature = "async")]
pub async fn fetch_data(resource_id: &str) -> anyhow::Result<String> {
    // Simulate network delay
    sleep(Duration::from_millis(100)).await;

    // Log with timestamp
    let timestamp = Utc::now();
    tracing::info!(
        time = timestamp.timestamp(),
        "Fetching resource: {}",
        resource_id
    );

    Ok(format!("Data for resource {}", resource_id))
}
