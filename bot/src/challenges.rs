use std::time::Duration;

use crate::channel::AppChannel;
use chrono::Timelike;

pub fn read_remaining_tiers() -> Option<u32> {
    let content =
        std::fs::read_to_string(poe_challenge_extractor::paths::remaining_tiers()).ok()?;
    let num = content.split_whitespace().last()?;
    num.parse().ok()
}

pub fn read_remaining_tiers_with_time() -> Option<(String, u32)> {
    let content =
        std::fs::read_to_string(poe_challenge_extractor::paths::remaining_tiers()).ok()?;
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.len() >= 2 {
        let time = parts[0].to_string();
        let remaining = parts.last()?.parse().ok()?;
        Some((time, remaining))
    } else {
        None
    }
}

pub async fn start_presence_updater(ctx: &poise::serenity_prelude::Context) {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;

        if let Some(remaining) = read_remaining_tiers() {
            let activity = poise::serenity_prelude::ActivityData::watching(format!(
                "{} frxtl tiers",
                remaining
            ));
            ctx.set_activity(Some(activity));
        }
    }
}

pub async fn start_daily_summarizer(ctx: &poise::serenity_prelude::Context) {
    use tokio::time::sleep;

    loop {
        // Check every minute if it's 22:00
        let now = chrono::Local::now();

        if now.hour() == 22 && now.minute() == 0 {
            if let Some((time, remaining)) = read_remaining_tiers_with_time() {
                let message = format!(
                    "Daily summary: <@407169521988665345> {} remaining tiers ({})",
                    remaining, time
                );
                AppChannel::Poe.say(ctx, &message).await;
            }
        }

        sleep(Duration::from_secs(60)).await;
    }
}
