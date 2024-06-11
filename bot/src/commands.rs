use chrono::FixedOffset;
use fresh_news::{Subforum, WebsiteLanguage};

use crate::{Context, Error};

/// Patchnotes links
#[poise::command(slash_command)]
pub async fn patchnotes(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://www.pathofexile.com/forum/view-thread/3496784\nhttps://ru.pathofexile.com/forum/view-thread/3496786")
        .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn news(ctx: Context<'_>) -> Result<(), Error> {
    let news = match fresh_news::get_fresh_threads(
        1500,
        &WebsiteLanguage::En,
        &Subforum::News,
        FixedOffset::east_opt(3600).as_ref(),
    )
    .await
    {
        Ok(threads) => threads
            .into_iter()
            .map(|thread| {
                println!("{} {thread:#?}", chrono::Local::now().format("%a %T"),);
                thread.url
            })
            .collect::<Vec<_>>()
            .join(" "),
        Err(err) => {
            ctx.say(format!("{err:?}")).await?;
            return Ok(());
        }
    };

    ctx.say(news).await?;
    Ok(())
}
