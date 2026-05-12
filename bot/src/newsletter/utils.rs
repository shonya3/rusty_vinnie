use crate::{
    channel::AppChannel,
    interval::{self},
};
use chrono::{DateTime, Utc};
use poise::serenity_prelude::Context as SerenityContext;
use std::error::Error;

pub trait Newsletter {
    type Item: NewsItem;
    type Error: Error;

    /// Fetches newsletter data from the source.
    /// Called automatically by [`fetch`](Self::fetch) with retry handling.
    async fn fetch_impl(&self) -> Result<Vec<Self::Item>, Self::Error>;

    /// Fetches with automatic retry (up to 3 attempts with 2s delay between).
    async fn fetch(&self) -> Result<Vec<Self::Item>, Self::Error> {
        let name = std::any::type_name::<Self>();
        let mut attempt = 0;

        loop {
            attempt += 1;
            match self.fetch_impl().await {
                Ok(items) => return Ok(items),
                Err(err) => {
                    eprintln!("{name} fetch attempt {attempt} failed: {err:?}");
                    if attempt == 3 {
                        return Err(err);
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            }
        }
    }

    async fn fetch_fresh(
        &self,
        stale_time: std::time::Duration,
    ) -> Result<Vec<Self::Item>, Self::Error> {
        let items = self.fetch().await?;
        let items = items
            .into_iter()
            .filter(|i| {
                interval::is_within_last_minutes((stale_time.as_secs() / 60) as i64, i.timestamp())
            })
            .collect();
        Ok(items)
    }

    #[allow(unused)]
    async fn send_fresh(
        &self,
        stale_time: std::time::Duration,
        ctx: &SerenityContext,
        channel: AppChannel,
    ) -> Result<(), Self::Error> {
        let items = self.fetch_fresh(stale_time).await?;

        for item in items {
            item.post_to_discord(ctx, channel).await;
        }

        Ok(())
    }

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
}

pub trait NewsItem {
    async fn post_to_discord(&self, ctx: &SerenityContext, channel: AppChannel);

    fn timestamp(&self) -> DateTime<Utc>;

    fn is_fresh(&self) -> bool {
        interval::is_fresh(self.timestamp())
    }
}
