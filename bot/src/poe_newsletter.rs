use crate::{channel::AppChannel, interval};
use chrono::FixedOffset;
use poe_forum::{Subforum, WebsiteLanguage};
use poise::serenity_prelude::{
    Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage,
    Timestamp,
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

// async fn watch_subforum(
//     ctx: &SerenityContext,
//     lang: WebsiteLanguage,
//     subforum: Subforum,
//     time_offset: Option<FixedOffset>,
// ) {
//     let mut interval =
//         tokio::time::interval(interval::duration_from_mins(interval::INTERVAL_MINS as u64));
//     let channel_id = AppChannel::Poe.id();

//     loop {
//         interval.tick().await;

//         match poe_forum::fetch_subforum_threads_list(lang, subforum, time_offset.as_ref()).await {
//             Ok(threads) => {
//                 let tasks = threads
//                     .into_iter()
//                     .filter(|thread| {
//                         interval::is_within_last_minutes(
//                             interval::INTERVAL_MINS,
//                             thread.posted_date,
//                         )
//                     })
//                     .map(|thread| {
//                         let embed = CreateEmbed::new()
//                             .author(CreateEmbedAuthor::new(subforum_title(lang, subforum)))
//                             .title(&thread.title)
//                             .url(&thread.url);

//                         channel_id.send_message(ctx, CreateMessage::new().embed(embed))
//                     })
//                     .collect::<Vec<_>>();

//                 for task in tasks {
//                     if let Err(err) = task.await {
//                         eprintln!("{err:?}");
//                     }
//                 }
//             }
//             Err(err) => eprintln!("{err:?}"),
//         }
//     }
// }

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
                    let mut embed = CreateEmbed::new()
                        .title(&thread.title)
                        .url(&thread.url)
                        .field(
                            "Posted date",
                            format!("<t:{}>", thread.posted_date.timestamp()),
                            false,
                        )
                        .footer(CreateEmbedFooter::new(subforum_title(lang, subforum)));

                    if let Some(author) = &thread.author {
                        embed = embed.author(CreateEmbedAuthor::new(author));
                    }

                    if let Ok(timestamp) =
                        Timestamp::from_millis(thread.posted_date.timestamp_millis())
                    {
                        embed = embed.timestamp(timestamp);
                    }

                    match fetch_post_html(&thread.url).await {
                        Ok(html) => {
                            let markdown = poe_forum_markdown::get_markdown(&html);
                            let markdown = truncate_to_max_chars(&markdown, 4095);

                            embed = embed.description(markdown);
                        }
                        Err(err) => eprintln!("Could not fetch post html {err}"),
                    }

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

pub async fn fetch_post_html(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()?;
    client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await
}

const USER_AGENT: &str = "rusty_vinnie/0.1 (contact: poeshonya3@gmail.com)";

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

#[allow(unused)]
pub async fn debug_send_embed(ctx: &SerenityContext) {
    let offset = FixedOffset::east_opt(3 * 3600);
    println!("Start loading list");

    let list = poe_forum::fetch_subforum_threads_list(
        WebsiteLanguage::En,
        Subforum::EarlyAccessPatchNotesEn,
        offset.as_ref(),
    )
    .await
    .unwrap();

    println!("Loaded list");

    // let info = list
    //     .iter()
    //     .find(|t| t.title == "0.2.0e Patch Notes")
    //     .unwrap();

    let info = list[0].clone();

    let patch_notes_html = fetch_post_html(&info.url).await.unwrap();

    let markdown = poe_forum_markdown::get_markdown(&patch_notes_html);
    let markdown = truncate_to_max_chars(&markdown, 4095);

    let millis = info.posted_date.timestamp_millis();

    let mut embed = CreateEmbed::new()
        .title(&info.title)
        .url(&info.url)
        .description(markdown)
        .field(
            "Posted date",
            format!("<t:{}>", info.posted_date.timestamp()),
            false,
        )
        .footer(CreateEmbedFooter::new(subforum_title(
            WebsiteLanguage::En,
            Subforum::EarlyAccessPatchNotesEn,
        )))
        .timestamp(Timestamp::from_millis(info.posted_date.timestamp_millis()).unwrap());
    if let Some(author) = &info.author {
        println!("Author is set {}", author);
        embed = embed.author(CreateEmbedAuthor::new(author));
    }

    let message = CreateMessage::new().embed(embed);
    println!("Message created. Sending...");
    AppChannel::Dev
        .id()
        .send_message(ctx, message)
        .await
        .unwrap();
}
