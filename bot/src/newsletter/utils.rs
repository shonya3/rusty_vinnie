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

pub trait Newsletter {
    type Item: NewsItem;
    type Error: Error;
    async fn fetch(&self) -> Result<Vec<Self::Item>, Self::Error>;

    async fn start(&self, ctx: &SerenityContext, channel: AppChannel) {
        let name = std::any::type_name::<Self>();
        let mut interval = interval::interval();
        loop {
            interval.tick().await;
            match self.fetch().await {
                Ok(items) => {
                    let items = items.into_iter().filter(|item| item.is_fresh());
                    for item in items {
                        item.post_to_discord(ctx, channel).await;
                    }
                }
                Err(err) => eprintln!("{name} error: {err:?}"),
            }
        }
    }

    #[allow(unused)]
    async fn fetch_fresh(
        &self,
        stale_time: std::time::Duration,
    ) -> Result<Vec<Self::Item>, Self::Error> {
        let vec = self.fetch().await?;
        let vec = vec
            .into_iter()
            .filter(|i| {
                interval::is_within_last_minutes((stale_time.as_secs() / 60) as i64, i.timestamp())
            })
            .collect();
        Ok(vec)
    }
}

pub trait NewsItem {
    async fn post_to_discord(&self, ctx: &SerenityContext, channel: AppChannel);

    fn timestamp(&self) -> DateTime<Utc>;

    fn is_fresh(&self) -> bool {
        interval::is_fresh(self.timestamp())
    }
}
