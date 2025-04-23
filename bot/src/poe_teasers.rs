use crate::{channel::AppChannel, Data};
use poe_teasers::{Teaser, TeasersForumThread};
use poise::serenity_prelude::{
    ChannelId, Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateMessage,
};
use std::{collections::HashSet, time::Duration};

pub async fn watch_teasers_threads(
    ctx: &SerenityContext,
    data: &Data,
    forum_threads: &[TeasersForumThread],
) {
    let mut interval = tokio::time::interval(Duration::from_secs(360));
    let channel_id = AppChannel::Poe.id();
    loop {
        interval.tick().await;
        for forum_thread in forum_threads {
            send_new_teasers(ctx, data, *forum_thread, channel_id).await;
        }
    }
}

async fn send_new_teasers(
    ctx: &SerenityContext,
    _data: &Data,
    forum_thread: TeasersForumThread,
    channel_id: ChannelId,
) {
    let thread_teasers = match poe_teasers::download_teasers_from_thread(forum_thread).await {
        Ok(teas) => teas,
        Err(err) => {
            println!("Could not download thread teasers. {err}");
            return;
        }
    };
    let published_teasers = load_published_teasers();

    for teaser in &thread_teasers {
        if !published_teasers.contains(teaser) {
            send_teaser(ctx, channel_id, teaser)
                .await
                .unwrap_or_else(|err| eprintln!("publish_new_teasers Error:{err}"))
        };
    }

    let mut set = HashSet::<Teaser>::from_iter(published_teasers);
    set.extend(thread_teasers);

    let _unique_teasers: Vec<Teaser> = set.into_iter().collect();

    if let Err(err) = save_published_teasers() {
        println!("Could not persist thread teasers: {err}");
    };
}

// TODO Use the actual storage
fn load_published_teasers() -> Vec<Teaser> {
    Vec::new()
}

// TODO Use the actual storage
fn save_published_teasers() -> Result<(), String> {
    Ok(())
}

async fn send_teaser(
    ctx: &SerenityContext,
    channel_id: ChannelId,
    teaser: &Teaser,
) -> Result<(), String> {
    let embed = CreateEmbed::new()
        .title(teaser.forum_thread.title())
        .url(teaser.forum_thread.url())
        .author(create_vinnie_bot_author_embed())
        .description(&teaser.heading);

    let images_embeds: Vec<CreateEmbed> = teaser
        .images_urls
        .iter()
        .map(|image_url| {
            CreateEmbed::new()
                .image(image_url)
                .url(teaser.forum_thread.url())
        })
        .collect();

    let mut embeds = vec![embed];
    embeds.extend(images_embeds);

    let message = CreateMessage::new().embeds(embeds);

    if let Err(err) = channel_id.send_message(&ctx, message).await {
        return Err(format!("Could not send teaser to {channel_id}. {err}"));
    }

    if !teaser.videos_urls.is_empty() {
        if let Err(err) = channel_id
            .send_message(
                &ctx,
                CreateMessage::new().content(teaser.videos_urls.join(" ")),
            )
            .await
        {
            return Err(format!("Could not send teaser to {channel_id}. {err}"));
        }
    }

    Ok(())
}

fn create_vinnie_bot_author_embed() -> CreateEmbedAuthor {
    CreateEmbedAuthor::new("Rusty Vinnie")
        .icon_url("https://discord.com/assets/ca24969f2fd7a9fb03d5.png")
        .url("https://github.com/shonya3/rusty_vinnie")
}
