use crate::{channel::AppChannel, interval, message::MessageWithThreadedDetails};
use chrono::FixedOffset;
use poe_forum::{post::PostDetails, NewsThreadInfo, Subforum, WebsiteLanguage};
use poise::serenity_prelude::{
    Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage,
    Timestamp,
};
use unicode_segmentation::UnicodeSegmentation;

pub async fn watch_subforums(
    ctx: &SerenityContext,
    configs: Vec<(WebsiteLanguage, Subforum)>,
    offset: Option<FixedOffset>,
) {
    futures::future::join_all(
        configs
            .into_iter()
            .map(|(lang, subforum)| watch_subforum(ctx, lang, subforum, offset)),
    )
    .await;
}

async fn watch_subforum(
    ctx: &SerenityContext,
    lang: WebsiteLanguage,
    subforum: Subforum,
    time_offset: Option<FixedOffset>,
) {
    let mut interval =
        tokio::time::interval(interval::duration_from_mins(interval::INTERVAL_MINS as u64));
    let channel_id = AppChannel::Poe.id();

    loop {
        interval.tick().await;

        match poe_forum::fetch_subforum_threads_list(lang, subforum, time_offset.as_ref()).await {
            Ok(threads) => {
                for thread in threads.into_iter().filter(|thread| {
                    interval::is_within_last_minutes(interval::INTERVAL_MINS, thread.posted_date)
                }) {
                    create_message(&thread).await.send(ctx, channel_id).await;
                }
            }
            Err(err) => eprintln!("{err:?}"),
        }
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
