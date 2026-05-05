use std::time::Duration;

use crate::channel::AppChannel;
use chrono::Timelike;
use poe_challenge_extractor::{load_history, TierEntry};

fn get_last_entry() -> Option<TierEntry> {
    load_history().entries.last().cloned()
}

#[allow(unused)]
pub async fn start_presence_updater(ctx: &poise::serenity_prelude::Context) {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;

        if let Some(entry) = get_last_entry() {
            let activity = poise::serenity_prelude::ActivityData::watching(format!(
                "{} frxtl tiers",
                entry.remaining
            ));
            ctx.set_activity(Some(activity));
        }
    }
}

pub async fn start_daily_summarizer(ctx: &poise::serenity_prelude::Context) {
    use tokio::time::sleep;

    loop {
        let now = chrono::Local::now();

        if now.hour() == 22 && now.minute() == 0 {
            if let Some(entry) = get_last_entry() {
                let hours_stale = entry.hours_since();
                let message = if hours_stale >= 24 {
                    format!(
                        "Daily summary: <@407169521988665345> {} tiers remaining ({} - no maps completed for {} hours)",
                        entry.remaining,
                        entry.datetime_moscow(),
                        hours_stale
                    )
                } else {
                    format!(
                        "Daily summary: <@407169521988665345> {} tiers remaining ({})",
                        entry.remaining,
                        entry.datetime_moscow()
                    )
                };
                AppChannel::Poe.say(ctx, &message).await;
            }
        }

        sleep(Duration::from_secs(60)).await;
    }
}
