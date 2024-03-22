use poise::serenity_prelude::{CacheHttp, ChannelId, CreateMessage};
use std::time::Duration;
pub const INTERVAL_MINS: i64 = 30;

pub async fn spin_news_loop(ctx: impl CacheHttp + 'static) {
    let mut interval = tokio::time::interval(Duration::from_secs(60 * INTERVAL_MINS as u64));
    let channel_id = ChannelId::new(356013349496029184);

    loop {
        interval.tick().await;
        match fresh_news::get_fresh_threads(INTERVAL_MINS, fresh_news::WebsiteLanguage::En).await {
            Ok(threads) => {
                let mut tasks = vec![];
                threads.into_iter().for_each(|thread| {
                    let task =
                        channel_id.send_message(&ctx, CreateMessage::new().content(thread.url));
                    tasks.push(task);
                });

                for task in tasks {
                    task.await.unwrap();
                }
            }
            Err(err) => eprintln!("{err:?}"),
        }
    }
}
