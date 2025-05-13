use crate::interval;

use last_epoch_forum::NewsThreadInfo;
pub use last_epoch_forum::Subforum;
use poise::serenity_prelude::{
    ChannelId, Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
    CreateMessage, Timestamp,
};
use unicode_segmentation::UnicodeSegmentation;

pub async fn watch_subforums(ctx: &SerenityContext, subforums: Vec<Subforum>) {
    let tasks = subforums
        .into_iter()
        .map(|subforum| watch_subforum(ctx, subforum))
        .collect::<Vec<_>>();

    futures::future::join_all(tasks).await;
}

async fn watch_subforum(ctx: &SerenityContext, subforum: Subforum) {
    let mut interval =
        tokio::time::interval(interval::duration_from_mins(interval::INTERVAL_MINS as u64));
    let channel_id = ChannelId::new(1362313267879350363);

    loop {
        interval.tick().await;
        match last_epoch_forum::fetch_subforum_threads_list(subforum).await {
            Ok(threads) => {
                for thread in threads.into_iter().filter(|thread| {
                    interval::is_within_last_minutes(interval::INTERVAL_MINS, thread.datetime)
                }) {
                    let embed = prepare_embed(thread).await;
                    if let Err(err) = channel_id
                        .send_message(ctx, CreateMessage::new().embed(embed))
                        .await
                    {
                        eprintln!("{err:?}");
                    }
                }
            }
            Err(err) => eprintln!("{err:?}"),
        }
    }
}

pub async fn prepare_embed(thread: NewsThreadInfo) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title(&thread.title)
        .url(&thread.url)
        .field(
            "Posted date",
            format!("<t:{}>", thread.datetime.timestamp()),
            true,
        )
        .footer(CreateEmbedFooter::new(subforum_title(thread.subforum)));

    if let Some(author) = &thread.author {
        embed = embed.author(CreateEmbedAuthor::new(author));
    }

    if let Ok(timestamp) = Timestamp::from_millis(thread.datetime.timestamp_millis()) {
        embed = embed.timestamp(timestamp);
    }

    if let Some(content) = &thread.content {
        embed = embed.field("Words", content.unicode_words().count().to_string(), true);
        let markdown = truncate_to_max_chars(content, 4095);
        embed = embed.description(markdown);
    }

    embed
}

pub fn subforum_title(subforum: Subforum) -> String {
    let (subforum_name, emoji) = match subforum {
        Subforum::Announcements => ("Announcements", "📢"),
        Subforum::News => ("News", "📰"),
        Subforum::DeveloperBlogs => ("Developer Blogs", "👨‍💻"),
        Subforum::PatchNotes => ("Patch Notes", "✏️"),
    };

    format!("{} {}", subforum_name, emoji)
}

fn truncate_to_max_chars(s: &str, max_chars: usize) -> String {
    let mut char_indices = s.char_indices();
    for _ in 0..max_chars {
        if char_indices.next().is_none() {
            return s.to_string();
        }
    }

    if let Some((idx, _)) = char_indices.next() {
        s[..idx].to_string()
    } else {
        s.to_string()
    }
}
