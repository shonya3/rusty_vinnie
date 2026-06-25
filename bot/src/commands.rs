use crate::{
    channel::AppChannel,
    newsletter::{NewsItem, Newsletter},
    time::fmt,
    PoiseContext,
};
use poise::{serenity_prelude::ChannelId, CreateReply};
use std::time::Duration;

pub type CommandError = Box<dyn std::error::Error + Send + Sync>;

/// Fetch fresh news from a specific newsletter
#[poise::command(slash_command)]
pub async fn fresh_news(
    ctx: PoiseContext<'_>,
    #[description = "Number of minutes to look back"] mins: u64,
    #[description = "Which newsletter: poe1, poe2, epoch, diablo"]
    newsletter_choice: NewsletterChoice,
) -> Result<(), CommandError> {
    ctx.defer_ephemeral().await?;

    match newsletter_choice {
        NewsletterChoice::Poe1 => {
            ctx.data()
                .newsletters
                .poe1
                .send_fresh(
                    Duration::from_secs(60 * mins),
                    ctx.serenity_context(),
                    ctx.channel_id(),
                )
                .await?
        }
        NewsletterChoice::Poe2 => {
            ctx.data()
                .newsletters
                .poe2
                .send_fresh(
                    Duration::from_secs(60 * mins),
                    ctx.serenity_context(),
                    ctx.channel_id(),
                )
                .await?
        }
        NewsletterChoice::LastEpoch => {
            ctx.data()
                .newsletters
                .epoch
                .send_fresh(
                    Duration::from_secs(60 * mins),
                    ctx.serenity_context(),
                    ctx.channel_id(),
                )
                .await?
        }
        NewsletterChoice::Diablo => {
            ctx.data()
                .newsletters
                .diablo
                .send_fresh(
                    Duration::from_secs(60 * mins),
                    ctx.serenity_context(),
                    ctx.channel_id(),
                )
                .await?
        }
    }

    ctx.reply("Done!").await?;

    Ok(())
}

#[derive(poise::ChoiceParameter)]
pub enum NewsletterChoice {
    #[name = "poe1"]
    Poe1,
    #[name = "poe2"]
    Poe2,
    #[name = "epoch"]
    LastEpoch,
    #[name = "diablo"]
    Diablo,
}

/// Post latest news from all newsletters
#[poise::command(slash_command)]
pub async fn news(
    ctx: PoiseContext<'_>,
    #[description = "Number of minutes to look back"] mins: u64,
    #[description = "Should all news items be posted in channels"] post: bool,
) -> Result<(), CommandError> {
    ctx.defer().await?;
    ctx.reply(format!(
        "Starting to collect {}news for the last {mins} mins",
        match post {
            true => "and post ",
            false => "",
        }
    ))
    .await?;
    let n = &ctx.data().newsletters;
    let stale_time = Duration::from_mins(mins);

    let p1 = news_per_newsletter(ctx, stale_time, &n.poe1, AppChannel::Poe1, post);
    let p2 = news_per_newsletter(ctx, stale_time, &n.poe2, AppChannel::Poe2, post);
    let e = news_per_newsletter(ctx, stale_time, &n.epoch, AppChannel::LastEpoch, post);
    let d = news_per_newsletter(ctx, stale_time, &n.diablo, AppChannel::Diablo, post);

    let (poe1_msg, poe2_msg, epoch_msg, diablo_msg) = tokio::join!(p1, p2, e, d);

    let mut msg = String::new();
    if !poe1_msg.is_empty() {
        msg += &format!("**Path of Exile 1**:\n{poe1_msg}");
    }
    if !poe2_msg.is_empty() {
        msg += &format!("\n\n**Path of Exile 2**:\n{poe2_msg}")
    }
    if !epoch_msg.is_empty() {
        msg += &format!("\n\n**Last Epoch**:\n{epoch_msg}")
    }
    if !diablo_msg.is_empty() {
        msg += &format!("\n\n**Diablo**:\n{diablo_msg}")
    }

    if msg.is_empty() {
        msg = format!("No news for the last {mins} mins.");
    }

    println!("{msg}");

    ctx.send(CreateReply::default().content(msg)).await?;

    Ok(())
}

/// Returns aggregated message about posts and posts each one if post param is true.
async fn news_per_newsletter<N, C>(
    ctx: PoiseContext<'_>,
    stale_time: Duration,
    newsletter: &N,
    channel: C,
    post: bool,
) -> String
where
    N: Newsletter,
    C: Into<ChannelId>,
{
    let items = match newsletter.fetch_fresh(stale_time).await {
        Ok(mut items) => {
            items.sort_by_key(|i| i.timestamp());
            items
        }
        Err(err) => {
            return format!("Could not fetch fresh items. {err:?}");
        }
    };

    let mut messages: Vec<String> = Vec::new();
    let channel_id = channel.into();
    for item in items {
        if post {
            item.post_to_discord(ctx.serenity_context(), channel_id)
                .await;
        }

        messages.push(format!(
            "\t{}: {}",
            fmt(item.timestamp(), true),
            item.title(),
        ));
    }

    messages.join("\n")
}
