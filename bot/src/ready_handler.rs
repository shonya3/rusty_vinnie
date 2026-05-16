use crate::{
    announcer::{self, Offset},
    channel::AppChannel,
    emoji::Emoji,
    newsletter::Newsletter,
    status::{get_kroiya_status, watch_status},
    Data, SerenityContext,
};
use chrono::{DateTime, NaiveDate, Utc};
use futures::future::join_all;
use poe_teasers::TeasersForumThread;
use rand::Rng;
use std::time::Duration;

// May 29th, 20:00 UTC
const LEAGUE_START: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(
    NaiveDate::from_ymd_opt(2026, 5, 29)
        .unwrap()
        .and_hms_opt(20, 0, 0)
        .unwrap(),
    Utc,
);

pub async fn handle_ready(ctx: &SerenityContext, data: &Data) {
    println!("Bot is ready");

    let secs = 60;
    println!("\nWatchers will start in {secs} seconds");
    for i in (1..=secs).rev() {
        println!("{i}...");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    println!("Set watchers");
    set_watchers(ctx, data).await;
}

async fn set_watchers(ctx: &SerenityContext, data: &Data) {
    let teasers = crate::poe_teasers::watch_teasers_threads(
        ctx,
        data,
        &[
            TeasersForumThread::Poe2_05(poe_teasers::Lang::En),
            TeasersForumThread::Poe2_05(poe_teasers::Lang::Ru),
        ],
        AppChannel::Poe2,
    );

    let presence_updater = async move {
        let mut interval = tokio::time::interval(Duration::from_mins(1));
        loop {
            interval.tick().await;
            announcer::update_presence(ctx, LEAGUE_START);
        }
    };

    tokio::join!(
        watch_status(
            || get_kroiya_status(ctx),
            || AppChannel::General.say(ctx, ":rabbit: пришел"),
            || AppChannel::General.say(ctx, ":rabbit: ушел"),
        ),
        teasers,
        data.newsletters.poe1.start(ctx, AppChannel::Poe1),
        data.newsletters.poe2.start(ctx, AppChannel::Poe2),
        data.newsletters.epoch.start(ctx, AppChannel::LastEpoch),
        data.newsletters.diablo.start(ctx, AppChannel::Diablo),
        league_start_announcer(ctx),
        presence_updater
    );
}

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
        (1..20)
            .map(|d| Offset::Days(d as i64))
            .chain([
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
