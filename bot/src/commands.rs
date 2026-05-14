use poise::CreateReply;

use crate::{channel::AppChannel, newsletter::Newsletter, Context};
use std::time::Duration;

pub type CommandError = Box<dyn std::error::Error + Send + Sync>;

/// Fetch fresh news from a specific newsletter
#[poise::command(slash_command)]
pub async fn fresh_news(
    ctx: Context<'_>,
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
pub async fn post_news(
    ctx: Context<'_>,
    #[description = "Number of minutes to look back"] mins: u64,
) -> Result<(), CommandError> {
    ctx.defer_ephemeral().await?;
    ctx.reply(format!("Starting to post news for the last {mins} mins"))
        .await?;
    let n = &ctx.data().newsletters;
    let context = ctx.serenity_context();
    let stale_time = Duration::from_mins(mins);

    let poe1 = n.poe1.send_fresh(stale_time, context, AppChannel::Poe1);
    let poe2 = n.poe2.send_fresh(stale_time, context, AppChannel::Poe2);
    let epoch = n
        .epoch
        .send_fresh(stale_time, context, AppChannel::LastEpoch);
    let diablo = n.diablo.send_fresh(stale_time, context, AppChannel::Diablo);

    let (poe1, poe2, epoch, diablo) = tokio::join!(poe1, poe2, epoch, diablo);

    poe1?;
    poe2?;
    epoch?;
    diablo?;

    ctx.send(CreateReply::default().content("Done!").ephemeral(true))
        .await?;

    Ok(())
}
