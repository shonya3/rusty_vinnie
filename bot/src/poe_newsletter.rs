use fresh_news::{Subforum, WebsiteLanguage};
use poise::serenity_prelude::{CacheHttp, ChannelId, CreateMessage};
use std::time::Duration;
pub const INTERVAL_MINS: i64 = 10;

fn mins_duration(mins: u64) -> Duration {
    Duration::from_secs(60 * mins)
}

pub async fn spin_news_loop(
    ctx: impl CacheHttp + 'static,
    lang: &WebsiteLanguage,
    subforum: &Subforum,
) {
    let mut interval = tokio::time::interval(mins_duration(INTERVAL_MINS as u64));
    let channel_id = ChannelId::new(356013349496029184);

    loop {
        interval.tick().await;
        match fresh_news::get_fresh_threads(INTERVAL_MINS, lang, subforum).await {
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
