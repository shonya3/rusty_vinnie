use crate::{channel::AppChannel, interval};
use diablo::{DiabloPost, PostKind};
use poise::serenity_prelude::{
    Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage,
    Timestamp,
};

pub async fn watch_diablo_news(ctx: &SerenityContext) {
    let mut interval =
        tokio::time::interval(interval::duration_from_mins(interval::INTERVAL_MINS as u64));
    let channel_id = AppChannel::Diablo.id();

    loop {
        interval.tick().await;

        match diablo::fetch_posts().await {
            Ok(posts) => {
                for post in posts.iter().filter(|post| {
                    interval::is_within_last_minutes(interval::INTERVAL_MINS, post.pub_date)
                        && !post.category.is_console_related()
                }) {
                    let message = CreateMessage::new().embed(create_summary_embed(post));
                    let _ = channel_id.send_message(ctx, message).await;
                }
            }
            Err(err) => eprintln!("{err:?}"),
        }
    }
}

pub fn create_summary_embed(post: &DiabloPost) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title(&post.title)
        .url(&post.url)
        .author(
            CreateEmbedAuthor::new(&post.user.name)
                .url(post.user.profile_url())
                .icon_url(&post.user.avatar_url),
        )
        .description(&post.description)
        .field(
            "Posted date",
            format!("<t:{}>", post.pub_date.timestamp()),
            true,
        );

    if let PostKind::News { post_image_url } = &post.kind {
        embed = embed.footer(CreateEmbedFooter::new("Diablo News 📢"));
        embed = embed.color(0x00c0ff);
        if let Some(image_url) = post_image_url {
            embed = embed.image(image_url);
        }
    } else {
        embed = embed.footer(CreateEmbedFooter::new("Diablo Post"));
    }

    if let Ok(timestamp) = Timestamp::from_millis(post.pub_date.timestamp_millis()) {
        embed = embed.timestamp(timestamp);
    }

    embed
}
