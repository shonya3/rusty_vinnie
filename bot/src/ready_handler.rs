use crate::{
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
    );
}
