use crate::{
    message::MessageWithThreadedDetails,
    newsletter::{NewsItem, Newsletter},
    time::Timezone,
};
use poe_forum::{post::PostDetails, NewsThreadInfo, Subforum, WebsiteLanguage};
use poise::serenity_prelude::{
    Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage,
    Timestamp,
};
use unicode_segmentation::UnicodeSegmentation;

pub struct PoeNewsletter {
    pub subforums: Vec<(WebsiteLanguage, Subforum)>,
    pub timezone: Timezone,
}

impl PoeNewsletter {
    pub fn new(subforums: Vec<(WebsiteLanguage, Subforum)>, timezone: Timezone) -> Self {
        Self {
            subforums,
            timezone,
        }
    }
}

impl Newsletter for PoeNewsletter {
    type Item = NewsThreadInfo;
    type Error = reqwest::Error;

    async fn fetch_impl(&self) -> Result<Vec<Self::Item>, Self::Error> {
        let mut all = Vec::new();
        for (lang, subforum) in &self.subforums {
            let items = poe_forum::fetch_subforum_threads_list(
                *lang,
                *subforum,
                self.timezone.offset().as_ref(),
            )
            .await?;
            all.extend(items);
        }
        Ok(all)
    }
}

impl NewsItem for NewsThreadInfo {
    async fn post_to_discord<C>(&self, ctx: &SerenityContext, channel: C)
    where
        C: Into<poise::serenity_prelude::ChannelId>,
    {
        create_message(self).await.send(ctx, channel.into()).await;
    }
    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        self.posted_date
    }

    fn title(&self) -> String {
        self.title.clone()
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
