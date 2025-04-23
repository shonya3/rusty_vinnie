use chrono::{DateTime, Duration as ChronoDuration, Utc};
use std::time::Duration;

pub const INTERVAL_MINS: i64 = 10;

pub fn duration_from_mins(mins: u64) -> Duration {
    Duration::from_secs(60 * mins)
}

pub fn is_within_last_minutes(minutes: i64, timestamp: DateTime<Utc>) -> bool {
    timestamp >= Utc::now() - ChronoDuration::minutes(minutes)
}
