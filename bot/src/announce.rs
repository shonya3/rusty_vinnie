use crate::{channel::AppChannel, emoji::Emoji, SerenityContext};
use chrono::{DateTime, Utc};
use rand::Rng;
use std::time::Duration;

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

pub async fn start_presence_updater(ctx: &SerenityContext, target: DateTime<Utc>) {
    let mut interval = tokio::time::interval(Duration::from_mins(1));
    loop {
        interval.tick().await;
        update_presence(ctx, target);
    }
}

pub fn event_offsets() -> impl Iterator<Item = Offset> {
    (2..20).map(|d| Offset::Days(d as i64)).chain([
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
}

pub fn generate_emojis() -> (String, String) {
    let mut emojis: Vec<String> = [
        "⏰", "🚨", "🐸", "🔥", "🎮", "✨", "🎉", "🚀", "🌟", "🔴", "💥", "⚡", "🌈", "😎", "🐺",
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

/// Add 6 unique emojis (3 to start, 3 to end).
pub fn with_emojis(s: &str) -> String {
    let (e1, e2) = generate_emojis();
    format!("{e1}{s}{e2}")
}

pub type Announcement = (AppChannel, Box<dyn Fn(Offset) -> String + Send>);

#[allow(unused)]
pub struct Announcer {
    /// Target date.
    date: chrono::DateTime<chrono::Utc>,
    offsets: Option<Vec<Offset>>,
    announcement: Option<Announcement>,
    presence: bool,
}

impl Announcer {
    pub fn new(date: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            date,
            offsets: None,
            announcement: None,
            presence: false,
        }
    }

    pub async fn start(self, ctx: &SerenityContext) {
        let date = self.date;
        let offsets = self.offsets;
        let announcement = self.announcement;
        let presence = self.presence;

        if !presence && announcement.is_none() {
            return;
        }

        let schedule = async {
            if let Some((channel, format)) = announcement {
                let offsets = offsets.unwrap_or_else(|| event_offsets().collect());
                futures::future::join_all(offsets.into_iter().filter(|o| o.is_upcoming(date)).map(
                    move |offset| {
                        let msg = format(offset);
                        async move {
                            offset
                                .schedule(date, move || async move {
                                    channel.say(ctx, &msg).await;
                                })
                                .await;
                        }
                    },
                ))
                .await;
            }
        };
        let presence_task = async {
            if presence {
                start_presence_updater(ctx, date).await;
            }
        };
        tokio::join!(schedule, presence_task);
    }

    #[allow(unused)]
    pub fn with_announcement(
        self,
        channel: AppChannel,
        format: impl Fn(Offset) -> String + Send + 'static,
    ) -> Self {
        Self {
            announcement: Some((channel, Box::new(format))),
            ..self
        }
    }

    #[allow(unused)]
    pub fn offsets(self, offsets: impl Iterator<Item = Offset>) -> Self {
        Self {
            offsets: Some(offsets.collect()),
            ..self
        }
    }

    #[allow(unused)]
    pub fn presence(self, presence: bool) -> Self {
        Self { presence, ..self }
    }
}
