use crate::{channel::AppChannel, interval};
use chrono::FixedOffset;
use fresh_news::{Subforum, WebsiteLanguage};
use poise::serenity_prelude::{
    Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateMessage,
};

pub async fn watch_subforums(
    ctx: &SerenityContext,
    configs: Vec<(WebsiteLanguage, Subforum)>,
    offset: Option<FixedOffset>,
) {
    let tasks = configs
        .into_iter()
        .map(|(lang, subforum)| watch_subforum(ctx, lang, subforum, offset))
        .collect::<Vec<_>>();

    futures::future::join_all(tasks).await;
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
        match fresh_news::fetch_subforum_threads_list(lang, subforum, time_offset.as_ref()).await {
            Ok(threads) => {
                let tasks = threads
                    .into_iter()
                    .filter(|thread| {
                        interval::is_within_last_minutes(
                            interval::INTERVAL_MINS,
                            thread.posted_date,
                        )
                    })
                    .map(|thread| {
                        let embed = CreateEmbed::new()
                            .author(CreateEmbedAuthor::new(subforum_title(lang, subforum)))
                            .title(&thread.title)
                            .url(&thread.url);

                        channel_id.send_message(ctx, CreateMessage::new().embed(embed))
                    })
                    .collect::<Vec<_>>();

                for task in tasks {
                    if let Err(err) = task.await {
                        eprintln!("{err:?}");
                    }
                }
            }
            Err(err) => eprintln!("{err:?}"),
        }
    }
}

fn subforum_title(lang: WebsiteLanguage, subforum: Subforum) -> String {
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
