use chrono::{DateTime, Duration as ChronoDuration, NaiveDate, Utc};

const ANNOUNCEMENT_TIMES: &[&str] = &["48h", "24h", "12h", "6h", "3h", "2h", "1h", "15min"];

pub const STREAM_DATETIME: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
    NaiveDate::from_ymd_opt(2026, 5, 7)
        .unwrap()
        .and_hms_opt(20, 0, 0)
        .unwrap(),
    Utc,
);

pub fn schedule_announcements(stream_starts_at: DateTime<Utc>) -> Vec<(String, DateTime<Utc>)> {
    ANNOUNCEMENT_TIMES
        .iter()
        .copied()
        .map(|label| {
            let offset_mins = match label {
                "48h" => 48 * 60,
                "24h" => 24 * 60,
                "12h" => 12 * 60,
                "6h" => 6 * 60,
                "3h" => 3 * 60,
                "2h" => 2 * 60,
                "1h" => 60,
                "15min" => 15,
                _ => unreachable!(),
            };
            let scheduled_at = stream_starts_at - ChronoDuration::minutes(offset_mins);
            (label.to_string(), scheduled_at)
        })
        .collect()
}

pub async fn start_announcer(
    ctx: poise::serenity_prelude::Context,
    channel: crate::channel::AppChannel,
) {
    use std::time::Duration;

    let mut interval = tokio::time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        update_presence(&ctx);
        check_announcements(&ctx, channel).await;
    }
}

fn update_presence(ctx: &poise::serenity_prelude::Context) {
    let now = Utc::now();
    let remaining = STREAM_DATETIME.signed_duration_since(now);
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

async fn check_announcements(ctx: &poise::serenity_prelude::Context, channel: crate::channel::AppChannel) {
    let now = Utc::now();
    let scheduled = schedule_announcements(STREAM_DATETIME);
    for (label, scheduled_at) in scheduled {
        if now.signed_duration_since(scheduled_at).num_seconds().abs() < 60 {
            let msg = format!("⏰ Stream starts in {}!", label);
            channel.say(ctx, &msg).await;
            println!("Sent announcement: {}", label);
        }
    }
}
