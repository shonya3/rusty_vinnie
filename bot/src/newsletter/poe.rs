use crate::{channel::AppChannel, message::MessageWithThreadedDetails, newsletter::NewsItem};
use poe_forum::{post::PostDetails, NewsThreadInfo, Subforum, WebsiteLanguage};
use poise::serenity_prelude::{
    Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage,
    Timestamp,
};
use unicode_segmentation::UnicodeSegmentation;

impl NewsItem for NewsThreadInfo {
    async fn post_to_discord(&self, ctx: &SerenityContext, channel: AppChannel) {
        create_message(self).await.send(ctx, channel.id()).await;
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        self.posted_date
    }
}

pub async fn create_message(thread: &NewsThreadInfo) -> MessageWithThreadedDetails {
    let post_details = http::text(&thread.url)
        .await
        .ok()
        .and_then(|html| poe_forum::get_post_details(&html));

    MessageWithThreadedDetails {
        message: CreateMessage::new().embed(create_summary_embed(thread, post_details.as_ref())),
        thread_name: thread.title.clone(),
        details_content: post_details.map(|post| post.content),
    }
}

pub fn create_summary_embed(
    thread: &NewsThreadInfo,
    post_details: Option<&PostDetails>,
) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title(&thread.title)
        .url(&thread.url)
        .field(
            "Posted date",
            format!("<t:{}>", thread.posted_date.timestamp()),
            true,
        )
        .footer(CreateEmbedFooter::new(subforum_title(
            thread.lang,
            thread.subforum,
        )));

    if let Some(author) = &thread.author {
        embed = embed.author(CreateEmbedAuthor::new(author));
    }

    if let Ok(timestamp) = Timestamp::from_millis(thread.posted_date.timestamp_millis()) {
        embed = embed.timestamp(timestamp);
    }

    if let Some(details) = post_details {
        embed = embed.field(
            "Words",
            details.content.unicode_words().count().to_string(),
            true,
        );

        if let Some(image_src) = &details.image_src {
            embed = embed.image(image_src);
        }
    }

    embed
}

pub fn subforum_title(lang: WebsiteLanguage, subforum: Subforum) -> String {
    let (subforum_name, emoji) = match subforum {
        Subforum::News => ("PoE News", "📢"),
        Subforum::PatchNotes => ("PoE Patch Notes", "✏️"),
        Subforum::EarlyAccessPatchNotesEn | Subforum::EarlyAccessPatchNotesRu => {
            ("PoE2 Patch Notes", "🆕")
        }
        Subforum::EarlyAccessAnnouncementsEn | Subforum::EarlyAccessAnnouncementsRu => {
            ("PoE2 Announcements", "📣")
        }
    };

    let lang_str = match lang {
        WebsiteLanguage::En => "EN",
        WebsiteLanguage::Ru => "RU",
    };

    format!("{} [{}] {}", subforum_name, lang_str, emoji)
}
