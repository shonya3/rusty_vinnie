use chrono::FixedOffset;
use fresh_news::{Subforum, WebsiteLanguage};
use poise::{
    serenity_prelude::{CreateEmbed, CreateEmbedAuthor},
    CreateReply,
};

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

fn create_vinnie_bot_author_embed() -> CreateEmbedAuthor {
    CreateEmbedAuthor::new("Rusty Vinnie")
        .icon_url("https://discord.com/assets/ca24969f2fd7a9fb03d5.png")
        .url("https://github.com/shonya3/rusty_vinnie")
}

pub async fn reply_with_ascendancies(ctx: Context<'_>, ascendancies: &[&str]) -> Result<(), Error> {
    let embed = CreateEmbed::new()
        .title("Path of Exile 2 - Ascendancy Classes in Early Access")
        .url("https://www.pathofexile.com/forum/view-thread/3592012")
        .author(create_vinnie_bot_author_embed())
        .description("At the start of Early Access in Path of Exile 2 each of the 6 Character Classes will have 2 Ascendancy Classes available. To help you choose your path, we’re revealing all 12 Ascendancies in this news post.");

    let images_embeds: Vec<CreateEmbed> = ascendancies
        .iter()
        .map(|image_url| {
            CreateEmbed::new()
                .image(*image_url)
                .url("https://www.pathofexile.com/forum/view-thread/3592012")
        })
        .collect();
    let mut reply = CreateReply::default().ephemeral(true);
    reply.embeds.push(embed);
    reply.embeds.extend(images_embeds);

    ctx.send(reply).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn ascendancies1(ctx: Context<'_>) -> Result<(), Error> {
    reply_with_ascendancies(
        ctx,
        &[
            "https://web.poecdn.com/public/news/2024-12-04/Stormweaver.png",
            "https://web.poecdn.com/public/news/2024-12-04/Chronomancer.png",
            "https://web.poecdn.com/public/news/2024-12-04/Titan.png",
            "https://web.poecdn.com/public/news/2024-12-04/Warbringer.png",
            "https://web.poecdn.com/public/news/2024-12-04/DEADEYE.jpg",
            "https://web.poecdn.com/public/news/2024-12-04/PATHFINDER.jpg",
        ],
    )
    .await
}

#[poise::command(slash_command)]
pub async fn ascendancies2(ctx: Context<'_>) -> Result<(), Error> {
    reply_with_ascendancies(
        ctx,
        &[
            "https://web.poecdn.com/public/news/2024-12-04/Bloodmage.png",
            "https://web.poecdn.com/public/news/2024-12-04/INFERNALIST.jpg",
            "https://web.poecdn.com/public/news/2024-12-04/Witchhunter.png",
            "https://web.poecdn.com/public/news/2024-12-04/GemlingLegionnaire.png",
            "https://web.poecdn.com/public/news/2024-12-04/INVOKER.jpg",
            "https://web.poecdn.com/public/news/2024-12-04/CHAYULA.jpg",
        ],
    )
    .await
}
