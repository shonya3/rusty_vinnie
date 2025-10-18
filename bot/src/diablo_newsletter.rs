use crate::{channel::AppChannel, newsletter::NewsItem};
use chrono::{DateTime, Utc};
use diablo::{DiabloPost, PostKind};
use poise::serenity_prelude::{
    Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage,
    Timestamp,
};

impl NewsItem for DiabloPost {
    async fn post_to_discord(&self, ctx: &SerenityContext, channel: AppChannel) {
        let message = CreateMessage::new().embed(create_summary_embed(self));
        let _ = channel.id().send_message(ctx, message).await;
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.pub_date
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
