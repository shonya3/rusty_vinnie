use chrono::{DateTime, Utc};
use std::time::Duration;

use crate::SerenityContext;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Offset {
    Days(i64),
    Hours(i64),
    Minutes(i64),
}

impl Offset {
    pub fn as_minutes(&self) -> i64 {
        match self {
            Offset::Days(d) => d * 1440,
            Offset::Hours(h) => h * 60,
            Offset::Minutes(m) => *m,
        }
    }

    pub fn label(&self) -> String {
        match self {
            Offset::Days(d) => format!("{d}d"),
            Offset::Hours(h) => format!("{h}h"),
            Offset::Minutes(m) => format!("{m}min"),
        }
    }

    /// Returns scheduled time for this offset
    pub fn time(&self, target: DateTime<Utc>) -> DateTime<Utc> {
        target - chrono::TimeDelta::minutes(self.as_minutes())
    }

    /// Returns true if announcement time is still in the future
    pub fn is_upcoming(&self, target: DateTime<Utc>) -> bool {
        self.time(target) > Utc::now()
    }

    pub async fn schedule<F, Fut>(&self, target: DateTime<Utc>, f: F)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let time = self.time(target);
        let delay = time.signed_duration_since(Utc::now());
        tokio::time::sleep(Duration::from_secs(delay.num_seconds() as u64)).await;
        f().await;
    }
}

#[allow(unused)]
pub fn update_presence(ctx: &SerenityContext, target: DateTime<Utc>) {
    let now = Utc::now();
    let remaining = target.signed_duration_since(now);
    if remaining.num_seconds() > 0 {
        let days = remaining.num_days();
        let hours = remaining.num_hours() % 24;
        let mins = remaining.num_minutes() % 60;
        let status = if days > 0 {
            format!("{}d {}h {}m", days, hours, mins)
        } else {
            format!("{}h {}m", hours, mins)
        };
        let activity = poise::serenity_prelude::ActivityData::watching(status);
        ctx.set_activity(Some(activity));
    }
}
