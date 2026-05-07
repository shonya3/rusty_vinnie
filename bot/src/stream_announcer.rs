use chrono::{DateTime, NaiveDate, Utc};
use std::time::Duration;

pub const STREAM_DATETIME: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
    NaiveDate::from_ymd_opt(2026, 5, 7)
        .unwrap()
        .and_hms_opt(20, 0, 0)
        .unwrap(),
    Utc,
);

#[derive(Debug, Clone, Copy)]
pub enum Offset {
    Hours(i64),
    Minutes(i64),
}

impl Offset {
    pub fn as_minutes(&self) -> i64 {
        match self {
            Offset::Hours(h) => h * 60,
            Offset::Minutes(m) => *m,
        }
    }

    pub fn label(&self) -> String {
        match self {
            Offset::Hours(h) => format!("{h}h"),
            Offset::Minutes(m) => format!("{m}min"),
        }
    }

    /// Returns scheduled time for this offset
    pub fn time(&self) -> DateTime<Utc> {
        STREAM_DATETIME - chrono::Duration::minutes(self.as_minutes())
    }

    /// Returns true if announcement time is still in the future
    pub fn is_upcoming(&self) -> bool {
        self.time() > Utc::now()
    }
}

pub async fn schedule<F, Fut>(offset: Offset, f: F)
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let time = offset.time();
    let delay = time.signed_duration_since(Utc::now());
    tokio::time::sleep(Duration::from_secs(delay.num_seconds() as u64)).await;
    f().await;
}
