use crate::{
    announce,
    channel::AppChannel,
    newsletter::Newsletter,
    status::{get_kroiya_status, watch_status},
    Data, SerenityContext,
};
use poe_teasers::TeasersForumThread;
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

    const STREAM_DATE: chrono::DateTime<chrono::Utc> =
        chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            chrono::NaiveDate::from_ymd_opt(2026, 7, 16)
                .unwrap()
                .and_hms_opt(20, 0, 0)
                .unwrap(),
            chrono::Utc,
        );

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
        announce::start_presence_updater(ctx, STREAM_DATE),
        futures::future::join_all(
            announce::event_offsets()
                .chain(std::iter::once(announce::Offset::Hours(29)))
                .filter(|o| o.is_upcoming(STREAM_DATE))
                .map(move |offset| async move {
                    offset
                        .schedule(STREAM_DATE, move || async move {
                            let (e1, e2) = announce::generate_emojis();
                            let msg = format!("{e1} Stream starts in {}! {e2}", offset.label());
                            AppChannel::Poe1.say(ctx, &msg).await;
                        })
                        .await;
                }),
        ),
    );
}
