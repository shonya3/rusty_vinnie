use poise::serenity_prelude::{CacheHttp, ChannelId, CreateMessage};
use std::time::Duration;

pub async fn spin_news_loop(ctx: impl CacheHttp + 'static) {
    let forever = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3600));

        loop {
            interval.tick().await;
            match fresh_news::get_fresh_news_url(fresh_news::WebsiteLanguage::En).await {
                Ok(option) => {
                    if let Some(url) = option {
                        ChannelId::new(356013349496029184)
                            .send_message(&ctx, CreateMessage::new().content(url.0))
                            .await
                            .unwrap();
                    }
                }
                Err(err) => eprintln!("{err:?}"),
            }
        }
    });

    forever.await.unwrap();
}
