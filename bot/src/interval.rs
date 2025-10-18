use chrono::{DateTime, Duration as ChronoDuration, Utc};
use std::{future::Future, time::Duration};
use tokio::time::Interval;

pub const INTERVAL_MINS: i64 = 10;

pub fn duration_from_mins(mins: u64) -> Duration {
    Duration::from_secs(60 * mins)
}

pub fn is_within_last_minutes(minutes: i64, timestamp: DateTime<Utc>) -> bool {
    timestamp >= Utc::now() - ChronoDuration::minutes(minutes)
}

pub fn interval() -> Interval {
    tokio::time::interval(duration_from_mins(INTERVAL_MINS as u64))
}

pub fn is_fresh(timestamp: DateTime<Utc>) -> bool {
    is_within_last_minutes(INTERVAL_MINS, timestamp)
}

/// Set interval with default [`INTERVAL_MINS`]
pub async fn set_interval<F, Fut>(f: F)
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    let mut interval = interval();

    loop {
        interval.tick().await;

        f().await;
    }
}
