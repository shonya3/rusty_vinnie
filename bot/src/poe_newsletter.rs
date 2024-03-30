use fresh_news::WebsiteLanguage;
use poise::serenity_prelude::{CacheHttp, ChannelId, CreateMessage};
use std::time::Duration;
pub const INTERVAL_MINS: i64 = 60;

pub async fn spin_news_loop(ctx: impl CacheHttp + 'static, lang: &WebsiteLanguage) {
    let mut interval = tokio::time::interval(Duration::from_secs(60 * INTERVAL_MINS as u64));
    let channel_id = ChannelId::new(356013349496029184);

    loop {
        interval.tick().await;
        match fresh_news::get_fresh_threads(INTERVAL_MINS, &lang).await {
            Ok(threads) => {
                let tasks = threads
                    .into_iter()
                    .map(|thread| {
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
