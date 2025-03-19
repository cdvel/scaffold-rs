use {{ project-name | snake_case }}::time_utils;

#[cfg(feature = "chrono")]
mod time_tests {
    use time_utils::{
        current_timestamp, parse_timestamp,
        add_days, days_between, from_unix_timestamp
    };
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_timestamp_operations() {
        // Generate current timestamp
        let ts = current_timestamp();

        // Parse it back
        let dt = parse_timestamp(&ts).unwrap();

        // Add days and calculate difference
        let future = add_days(dt, 7);
        let days = days_between(dt, future);

        assert_eq!(days, 7);
    }

    #[test]
    fn test_unix_timestamp() {
        // January 1, 2023 00:00:00 UTC
        let ts = 1672531200;
        let dt = from_unix_timestamp(ts).unwrap();

        // Verify the conversion is correct
        let expected = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(dt, expected);
    }
}
