use crate::{
    announce::{with_emojis, Announcer},
    channel::AppChannel,
    newsletter::Newsletter,
    status::{get_kroiya_status, watch_status, Status},
    Data, SerenityContext,
};
use chrono::{DateTime, NaiveDate, Utc};
use std::time::Duration;

pub async fn handle_ready(ctx: &SerenityContext, data: &Data) {
    println!("Bot is ready");

    let secs = 60;
    println!("\nWatchers will start in {secs} seconds");
    for i in (1..=secs).rev() {
        println!("{i}...");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    println!("Set watchers");
    start_watchers(ctx, data).await;
}

async fn start_watchers(ctx: &SerenityContext, data: &Data) {
    let stream = Announcer::new(DateTime::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(2026, 7, 16)
            .unwrap()
            .and_hms_opt(20, 0, 0)
            .unwrap(),
        Utc,
    ))
    .announcement(AppChannel::Poe1, |offset| {
        with_emojis(&format!(" Stream starts in {}! ", offset.label()))
    })
    .start(ctx);

    let league = Announcer::new(DateTime::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(2026, 7, 24)
            .unwrap()
            .and_hms_opt(20, 0, 0)
            .unwrap(),
        Utc,
    ))
    .announcement(AppChannel::Poe1, |offset| {
        with_emojis(&format!(" 3.29 League starts in {}! ", offset.label()))
    })
    .presence(true)
    .start(ctx);

    tokio::join!(
        stream,
        league,
        watch_status(
            || get_kroiya_status(ctx),
            |status| match status {
                Status::Online => AppChannel::General.say(ctx, ":rabbit: пришел"),
                Status::Offline => AppChannel::General.say(ctx, ":rabbit: ушел"),
            },
        ),
        data.newsletters.poe1.start(ctx, AppChannel::Poe1),
        data.newsletters.poe2.start(ctx, AppChannel::Poe2),
        data.newsletters.epoch.start(ctx, AppChannel::LastEpoch),
        data.newsletters.diablo.start(ctx, AppChannel::Diablo),
    );
}
