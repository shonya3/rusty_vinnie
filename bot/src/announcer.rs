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

#[allow(unused)]
mod league_start {
    use crate::{announcer::Offset, channel::AppChannel, emoji::Emoji, SerenityContext};
    use chrono::{DateTime, NaiveDate, Utc};
    use futures::future::join_all;
    use rand::Rng;

    // May 29th, 20:00 UTC
    const LEAGUE_START: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(2026, 5, 29)
            .unwrap()
            .and_hms_opt(20, 0, 0)
            .unwrap(),
        Utc,
    );

    fn presence() {
        // let presence_updater = async move {
        //     let mut interval = tokio::time::interval(Duration::from_mins(1));
        //     loop {
        //         interval.tick().await;
        //         announcer::update_presence(ctx, LEAGUE_START);
        //     }
        // };
    }

    #[allow(unused)]
    async fn league_start_announcer(ctx: &SerenityContext) {
        fn generate_emojis() -> (String, String) {
            let mut emojis: Vec<String> = [
                "⏰", "🚨", "🐸", "🔥", "🎮", "✨", "🎉", "🚀", "🌟", "🔴", "💥", "⚡", "🌈", "😎",
                "🐺",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .chain(Emoji::all().into_iter().map(|e| e.to_string()))
            .collect();

            let mut rng = rand::rng();
            let mut pick = || emojis.swap_remove(rng.random_range(0..emojis.len()));

            (
                format!("{}{}{}", pick(), pick(), pick()),
                format!("{}{}{}", pick(), pick(), pick()),
            )
        }

        join_all(
            (2..20)
                .map(|d| Offset::Days(d as i64))
                .chain([
                    Offset::Hours(30),
                    Offset::Hours(27),
                    Offset::Hours(24),
                    Offset::Hours(20),
                    Offset::Hours(16),
                    Offset::Hours(12),
                    Offset::Hours(10),
                    Offset::Hours(8),
                    Offset::Hours(6),
                    Offset::Hours(5),
                    Offset::Hours(4),
                    Offset::Hours(3),
                    Offset::Hours(2),
                    Offset::Hours(1),
                    Offset::Minutes(45),
                    Offset::Minutes(30),
                    Offset::Minutes(15),
                    Offset::Minutes(10),
                    Offset::Minutes(5),
                    Offset::Minutes(2),
                    Offset::Minutes(1),
                ])
                .filter(|o| o.is_upcoming(LEAGUE_START))
                .map(move |offset| async move {
                    offset
                        .schedule(LEAGUE_START, move || async move {
                            let (e1, e2) = generate_emojis();
                            let msg = format!("{e1} League starts in {}! {e2}", offset.label());
                            AppChannel::Poe2.say(ctx, &msg).await;
                        })
                        .await;
                }),
        )
        .await;
    }
}
