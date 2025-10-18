use crate::{
    channel::AppChannel,
    interval::{self, set_interval},
};
use chrono::{DateTime, Utc};
use poise::serenity_prelude::Context as SerenityContext;
use std::{error::Error, future::Future};

pub async fn start_news_feed<T, Fut, F, E>(
    ctx: &SerenityContext,
    channel: AppChannel,
    fetch_items: F,
) where
    T: NewsItem,
    Fut: Future<Output = Result<Vec<T>, E>> + Send,
    F: Fn() -> Fut,
    E: Error,
{
    set_interval(async || match fetch_items().await {
        Ok(items) => {
            let items = items.into_iter().filter(|item| item.is_fresh());
            for item in items {
                item.post_to_discord(ctx, channel).await;
            }
        }
        Err(err) => eprintln!("{err:?}"),
    })
    .await;
}

pub trait NewsItem {
    async fn post_to_discord(&self, ctx: &SerenityContext, channel: AppChannel);

    fn timestamp(&self) -> DateTime<Utc>;

    fn is_fresh(&self) -> bool {
        interval::is_fresh(self.timestamp())
    }
}
