use crate::interval;

pub use last_epoch_news::Subforum;
use poise::serenity_prelude::{ChannelId, Context as SerenityContext, CreateMessage};

pub async fn watch_subforums(ctx: &SerenityContext, subforums: Vec<Subforum>) {
    let tasks = subforums
        .into_iter()
        .map(|subforum| watch_subforum(ctx, subforum))
        .collect::<Vec<_>>();

    futures::future::join_all(tasks).await;
}

async fn watch_subforum(ctx: &SerenityContext, subforum: Subforum) {
    let mut interval =
        tokio::time::interval(interval::duration_from_mins(interval::INTERVAL_MINS as u64));
    let channel_id = ChannelId::new(1362313267879350363);

    loop {
        interval.tick().await;
        match last_epoch_news::fetch_subforum_threads_list(subforum).await {
            Ok(threads) => {
                let content = threads
                    .into_iter()
                    .filter(|thread| {
                        interval::is_within_last_minutes(interval::INTERVAL_MINS, thread.datetime)
                    })
                    .map(|thread| thread.url)
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
