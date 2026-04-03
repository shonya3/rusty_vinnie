use std::time::Duration;

const TIERS_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\..\\remaining_tiers.txt");

pub fn read_remaining_tiers() -> Option<u32> {
    let content = std::fs::read_to_string(TIERS_FILE).ok()?;
    let num = content.trim().split_whitespace().last()?;
    num.parse().ok()
}

pub async fn start_presence_updater(ctx: &poise::serenity_prelude::Context) {
    let mut interval = tokio::time::interval(Duration::from_secs(5));
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
