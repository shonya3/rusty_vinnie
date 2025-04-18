use chrono::{DateTime, Duration as ChronoDuration, Utc};
pub use last_epoch_news::Subforum;
use poise::serenity_prelude::{ChannelId, Context as SerenityContext, CreateMessage};
use std::time::Duration;
pub const INTERVAL_MINS: i64 = 10;
fn mins_duration(mins: u64) -> Duration {
    Duration::from_secs(60 * mins)
}

fn is_within_last_minutes(minutes: i64, timestamp: DateTime<Utc>) -> bool {
    timestamp >= Utc::now() - ChronoDuration::minutes(minutes)
}

pub async fn watch_lastepoch(ctx: &SerenityContext, subforum: Subforum) {
    let mut interval = tokio::time::interval(mins_duration(INTERVAL_MINS as u64));
    let channel_id = ChannelId::new(1362313267879350363);

    loop {
        interval.tick().await;
        match last_epoch_news::fetch_subforum_threads_list(subforum).await {
            Ok(threads) => {
                println!("{:?}", threads);
                let content = threads
                    .into_iter()
                    .filter(|thread| is_within_last_minutes(INTERVAL_MINS, thread.datetime))
                    .map(|thread| {
                        println!("{} {thread:#?}", chrono::Local::now().format("%a %T"),);
                        thread.url
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                channel_id
                    .send_message(&ctx, CreateMessage::new().content(content))
                    .await
                    .ok();
            }
            Err(err) => eprintln!("{err:?}"),
        }
    }
}
