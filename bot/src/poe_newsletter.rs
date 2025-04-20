use chrono::FixedOffset;
use fresh_news::{Subforum, WebsiteLanguage};
use poise::serenity_prelude::{Context as SerenityContext, CreateMessage};
use std::time::Duration;

use crate::channel::AppChannel;
pub const INTERVAL_MINS: i64 = 10;
fn mins_duration(mins: u64) -> Duration {
    Duration::from_secs(60 * mins)
}

pub async fn watch_subforums(
    ctx: &SerenityContext,
    configs: Vec<(WebsiteLanguage, Subforum)>,
    offset: Option<FixedOffset>,
) {
    let tasks = configs
        .into_iter()
        .map(|(lang, subforum)| watch_subforum(ctx, lang, subforum, offset))
        .collect::<Vec<_>>();

    futures::future::join_all(tasks).await;
}

async fn watch_subforum(
    ctx: &SerenityContext,
    lang: WebsiteLanguage,
    subforum: Subforum,
    time_offset: Option<FixedOffset>,
) {
    let mut interval = tokio::time::interval(mins_duration(INTERVAL_MINS as u64));
    let channel_id = AppChannel::Poe.id();

    loop {
        interval.tick().await;
        match fresh_news::get_fresh_threads(INTERVAL_MINS, lang, subforum, time_offset.as_ref())
            .await
        {
            Ok(threads) => {
                let tasks = threads
                    .into_iter()
                    .map(|thread| {
                        println!("{} {thread:#?}", chrono::Local::now().format("%a %T"),);
                        channel_id.send_message(&ctx, CreateMessage::new().content(thread.url))
                    })
                    .collect::<Vec<_>>();

                for task in tasks {
                    if let Err(err) = task.await {
                        eprintln!("{err:?}");
                    }
                }
            }
            Err(err) => eprintln!("{err:?}"),
        }
    }
}
