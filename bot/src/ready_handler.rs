use std::time::Duration;

use crate::{
    challenges,
    channel::AppChannel,
    newsletter::Newsletter,
    status::{get_kroiya_status, watch_status},
    stream_announcer::{self, Offset},
    Data,
};
use futures::future::join_all;
use poe_teasers::TeasersForumThread;
use poise::serenity_prelude::{self as serenity};
use rand::seq::IndexedRandom;

pub async fn handle_ready(ctx: &serenity::Context, data: &Data) {
    println!("Bot is ready");

    println!("\nWatchers will start in 10 seconds");
    for i in (1..=10).rev() {
        println!("{i}...");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    println!("Set watchers");
    set_watchers(ctx, data).await;
}

async fn set_watchers(ctx: &serenity::Context, data: &Data) {
    let teasers = crate::poe_teasers::watch_teasers_threads(
        ctx,
        data,
        &[
            TeasersForumThread::Poe2_05(poe_teasers::Lang::En),
            TeasersForumThread::Poe2_05(poe_teasers::Lang::Ru),
        ],
        AppChannel::Poe2,
    );

    tokio::join!(
        watch_status(
            || get_kroiya_status(ctx),
            || AppChannel::General.say(ctx, ":rabbit: пришел"),
            || AppChannel::General.say(ctx, ":rabbit: ушел"),
        ),
        teasers,
        data.newsletters.poe1.start(ctx, AppChannel::Poe),
        data.newsletters.poe2.start(ctx, AppChannel::Poe2),
        data.newsletters
            .last_epoch
            .start(ctx, AppChannel::LastEpoch),
        data.newsletters.diablo.start(ctx, AppChannel::Diablo),
        challenges::start_daily_summarizer(ctx),
    );
}

#[allow(unused)]
fn countdown(ctx: &serenity::Context) {
    let e = || {
        [
            "⏰", "🚨", "🐸", "🔥", "🎮", "✨", "🎉", "🚀", "🌟", "🔴", "💥", "⚡", "🌈", "🐭",
            "🤓", "😎", "🦀",
        ]
        .choose(&mut rand::rng())
        .unwrap()
    };

    let stream_announcer = join_all(
        [
            Offset::Hours(48),
            Offset::Hours(24),
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
        ]
        .into_iter()
        .filter(|o| o.is_upcoming())
        .map(|offset| async move {
            offset
                .schedule(move || async move {
                    let e1 = format!("{}{}{}", e(), e(), e());
                    let e2 = format!("{}{}{}", e(), e(), e());
                    let msg = format!("{e1} Stream starts in {} {e2}", offset.label());
                    AppChannel::Poe2.say(&ctx, &msg).await;
                })
                .await;
        }),
    );
}

#[allow(unused)]
fn precence(ctx: &serenity::Context) {
    let presence_updater = async move {
        let mut interval = tokio::time::interval(Duration::from_mins(1));
        loop {
            interval.tick().await;
            stream_announcer::update_presence(ctx);
        }
    };
}
