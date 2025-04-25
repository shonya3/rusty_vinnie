use crate::{channel::AppChannel, interval};
use chrono::FixedOffset;
use poe_forum::{NewsThreadInfo, Subforum, WebsiteLanguage};
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
                    let embed = prepare_embed(thread, lang, subforum).await;

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

pub async fn prepare_embed(
    thread: NewsThreadInfo,
    lang: WebsiteLanguage,
    subforum: Subforum,
) -> CreateEmbed {
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

    if let Ok(timestamp) = Timestamp::from_millis(thread.posted_date.timestamp_millis()) {
        embed = embed.timestamp(timestamp);
    }

    match fetch_post_html(&thread.url).await {
        Ok(html) => {
            if let Some(details) = poe_forum_markdown::get_details(&html) {
                let markdown = truncate_to_max_chars(&details.content, 4095);
                embed = embed.description(markdown);

                if let Some(image_src) = &details.image_src {
                    embed = embed.image(image_src);
                }
            }
        }
        Err(err) => eprintln!("Could not fetch post html {err}"),
    };

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
pub mod debug {
    use chrono::FixedOffset;
    use poe_forum::{NewsThreadInfo, Subforum, WebsiteLanguage};

    pub fn offset() -> Option<FixedOffset> {
        FixedOffset::east_opt(3 * 3600)
    }

    pub enum Threads {
        News020e,
    }

    impl Threads {
        pub fn thread(&self) -> NewsThreadInfo {
            match self {
                Threads::News020e => NewsThreadInfo {
                    url: "https://www.pathofexile.com/forum/view-thread/3765101".to_owned(),
                    posted_date: "2025-04-17T08:28:24Z".parse().unwrap(),
                    title: "Upcoming Plans for 0.2.0g".to_owned(),
                    author: Some("Community_Team".to_owned()),
                },
            }
        }

        pub fn lang(&self) -> WebsiteLanguage {
            match self {
                Threads::News020e => WebsiteLanguage::En,
            }
        }

        pub fn subforum(&self) -> Subforum {
            match self {
                Threads::News020e => Subforum::EarlyAccessAnnouncementsEn,
            }
        }
    }
}
