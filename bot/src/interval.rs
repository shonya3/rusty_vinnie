use chrono::{DateTime, TimeDelta, Utc};
use std::{future::Future, time::Duration};
use tokio::time::Interval;

pub const INTERVAL_MINS: i64 = 10;

pub fn is_within_last_minutes(minutes: i64, timestamp: DateTime<Utc>) -> bool {
    timestamp >= Utc::now() - TimeDelta::minutes(minutes)
}

pub fn interval() -> Interval {
    tokio::time::interval(Duration::from_mins(INTERVAL_MINS as u64))
}

pub fn is_fresh(timestamp: DateTime<Utc>) -> bool {
    is_within_last_minutes(INTERVAL_MINS, timestamp)
}

#[allow(unused)]
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
