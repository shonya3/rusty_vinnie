use crate::{Context, Error};

/// Patchnotes links
#[poise::command(slash_command)]
pub async fn patchnotes(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://www.pathofexile.com/forum/view-thread/3496784\nhttps://ru.pathofexile.com/forum/view-thread/3496786")
        .await?;
    Ok(())
}
