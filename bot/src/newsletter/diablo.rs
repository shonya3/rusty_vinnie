use crate::{
    newsletter::{NewsItem, Newsletter},
    SerenityContext,
};
use chrono::{DateTime, Utc};
use diablo::{DiabloPost, PostKind};
use poise::serenity_prelude::{
    CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, Timestamp,
};

pub struct DiabloNewsletter;

impl Newsletter for DiabloNewsletter {
    type Item = DiabloPost;
    type Error = diablo::Error;

    async fn fetch_impl(&self) -> Result<Vec<Self::Item>, Self::Error> {
        let posts = diablo::fetch_posts().await?;
        Ok(posts
            .into_iter()
            .filter(|post| !post.category.is_console_related())
            .collect())
    }
}

impl NewsItem for DiabloPost {
    async fn post_to_discord<C>(&self, ctx: &SerenityContext, channel: C)
    where
        C: Into<poise::serenity_prelude::ChannelId>,
    {
        let message = CreateMessage::new().embed(create_summary_embed(self));
        let _ = channel.into().send_message(ctx, message).await;
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.pub_date
    }

    fn title(&self) -> String {
        self.title.clone()
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
