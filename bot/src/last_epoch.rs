use crate::{interval, Context, Error};

use last_epoch_forum::NewsThreadInfo;
pub use last_epoch_forum::Subforum;
use poise::{
    serenity_prelude::{
        ChannelId, Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
        CreateMessage, Timestamp,
    },
    CreateReply,
};
use unicode_segmentation::UnicodeSegmentation;

pub async fn watch_subforums(ctx: &SerenityContext, subforums: Vec<Subforum>) {
    futures::future::join_all(
        subforums
            .into_iter()
            .map(|subforum| watch_subforum(ctx, subforum)),
    )
    .await;
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

        embed = embed.description(
            content
                .chars()
                .take(crate::EMBED_DESCRIPTION_CUSTOM_MAX_CHARS)
                .collect::<String>(),
        );
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

/// Fetch a specific thread from the Last Epoch forums
#[poise::command(slash_command)]
pub async fn epoch_thread(
    ctx: Context<'_>,
    #[description = "Select a subforum"] subforum: SubforumSlash,
    #[description = "Nth thread to fetch"]
    #[min = 1]
    #[max = 3]
    nth: usize,
) -> Result<(), Error> {
    ctx.defer().await?;

    match last_epoch_forum::fetch_subforum_threads_list(subforum.into()).await {
        Ok(threads) => {
            if let Some(thread) = threads.into_iter().filter(|t| !t.is_pinned).nth(nth - 1) {
                // if let Some(thread) = threads.into_iter().nth(nth - 1) {
                let embed = prepare_embed(thread).await;
                ctx.send(CreateReply::default().embed(embed)).await.unwrap();
            } else {
                ctx.say("Not found").await?;
            }
        }
        Err(err) => {
            ctx.say(format!("Failed to fetch threads {err}")).await?;
        }
    };

    Ok(())
}

#[derive(poise::ChoiceParameter)]
enum SubforumSlash {
    #[name = "announcements"]
    Announcements,
    #[name = "news"]
    News,
    #[name = "developer_blogs"]
    DeveloperBlogs,
    #[name = "patch_notes"]
    PatchNotes,
}

impl From<SubforumSlash> for Subforum {
    fn from(value: SubforumSlash) -> Self {
        match value {
            SubforumSlash::Announcements => Subforum::Announcements,
            SubforumSlash::News => Subforum::News,
            SubforumSlash::DeveloperBlogs => Subforum::DeveloperBlogs,
            SubforumSlash::PatchNotes => Subforum::PatchNotes,
        }
    }
}
