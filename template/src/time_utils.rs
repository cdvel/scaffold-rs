//! Time handling utilities
//!
//! Provides common time operations using the chrono crate.

#[cfg(feature = "chrono")]
use anyhow::Result;
#[cfg(feature = "chrono")]
use chrono::{DateTime, Duration, NaiveDateTime, Utc};

/// Returns the current UTC timestamp in RFC 3339 format
#[cfg(feature = "chrono")]
pub fn current_timestamp() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.to_rfc3339()
}

/// Parses a timestamp string in RFC 3339 format
#[cfg(feature = "chrono")]
pub fn parse_timestamp(timestamp: &str) -> Result<DateTime<Utc>> {
    let parsed = DateTime::parse_from_rfc3339(timestamp)?;
    Ok(parsed.with_timezone(&Utc))
}

/// Adds days to a timestamp
#[cfg(feature = "chrono")]
pub fn add_days(timestamp: DateTime<Utc>, days: i64) -> DateTime<Utc> {
    timestamp + Duration::days(days)
}

/// Calculates the difference in days between two timestamps
#[cfg(feature = "chrono")]
pub fn days_between(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
    let duration = end.signed_duration_since(start);
    duration.num_days()
}

/// Convert a Unix timestamp to DateTime
#[cfg(feature = "chrono")]
#[cfg(feature = "chrono")]
pub fn from_unix_timestamp(secs: i64) -> Result<DateTime<Utc>> {
    DateTime::from_timestamp(secs, 0).ok_or_else(|| anyhow::anyhow!("Invalid timestamp"))
}

/// Format a DateTime using a custom format string
#[cfg(feature = "chrono")]
pub fn format_datetime(dt: DateTime<Utc>, fmt: &str) -> String {
    dt.format(fmt).to_string()
}
