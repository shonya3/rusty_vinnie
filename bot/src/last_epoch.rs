use crate::{message::MessageWithThreadedDetails, newsletter::NewsItem, Context, Error};

use last_epoch_forum::NewsThreadInfo;
pub use last_epoch_forum::Subforum;
use poise::serenity_prelude::{
    Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage,
    Timestamp,
};
use unicode_segmentation::UnicodeSegmentation;

impl NewsItem for NewsThreadInfo {
    async fn post_to_discord(&self, ctx: &SerenityContext, channel: crate::channel::AppChannel) {
        create_message(self).send(ctx, channel.id()).await
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        self.datetime
    }
}

pub fn create_message(thread: &NewsThreadInfo) -> MessageWithThreadedDetails {
    MessageWithThreadedDetails {
        message: CreateMessage::new().embed(create_summary_embed(thread)),
        thread_name: thread.title.clone(),
        details_content: thread.content.clone(),
    }
}

pub fn create_summary_embed(thread: &NewsThreadInfo) -> CreateEmbed {
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
    ctx.defer_ephemeral().await?;
    match last_epoch_forum::fetch_subforum_threads_list(subforum.into()).await {
        Ok(threads) => {
            if let Some(thread) = threads.into_iter().nth(nth - 1) {
                create_message(&thread)
                    .send(ctx.serenity_context(), ctx.channel_id())
                    .await;
                ctx.say("Done !").await?;
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
