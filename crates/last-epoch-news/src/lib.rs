use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const USER_AGENT: &str = "rusty_vinnie/0.1 (contact: poeshonya3@gmail.com)";

pub async fn fetch_subforum_threads_list(
    subforum: Subforum,
) -> Result<Vec<NewsThreadInfo>, reqwest::Error> {
    let client = reqwest::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()?;
    let html = client
        .get(format!("https://forum.lastepoch.com/c/{subforum}"))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(html::prepare_threads_info(&html).await)
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct NewsThreadInfo {
    pub url: String,
    pub title: String,
    pub unix_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub enum Subforum {
    Announcements,
    News,
    DeveloperBlogs,
    PatchNotes,
}

impl std::fmt::Display for Subforum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Subforum::Announcements => "announcements",
            Subforum::News => "news",
            Subforum::DeveloperBlogs => "developer-blogs",
            Subforum::PatchNotes => "patch-notes",
        };

        f.write_str(s)
    }
}

pub mod html {
    use crate::NewsThreadInfo;
    use chrono::{DateTime, Utc};
    use futures::stream::{self, StreamExt};
    use scraper::{ElementRef, Html, Selector};

    async fn fetch_post_markup(url: &str) -> Result<String, reqwest::Error> {
        let client = reqwest::ClientBuilder::new()
            .user_agent(super::USER_AGENT)
            .build()?;
        let html = client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(html)
    }

    pub async fn prepare_threads_info(html: &str) -> Vec<NewsThreadInfo> {
        let document = Html::parse_document(html);
        let tr_selector = create_selector("table tbody tr");
        let rows = document.select(&tr_selector).take(3);

        let tasks: Vec<_> = rows
            .filter_map(|row| {
                let url = get_thread_url(&row)?;
                let title = get_thread_title(&row)?;
                Some((url, title))
            })
            .collect();

        stream::iter(tasks)
            .map(|(url, title)| async move {
                let post_markup = fetch_post_markup(&url).await.ok()?;
                let unix_timestamp = get_unix_timestamp(&post_markup)?;
                Some(NewsThreadInfo {
                    url,
                    title,
                    unix_timestamp,
                })
            })
            .buffer_unordered(3)
            .filter_map(|x| async move { x })
            .collect()
            .await
    }

    fn title_selector() -> Selector {
        create_selector("a.title")
    }

    fn create_selector(selectors: &str) -> Selector {
        Selector::parse(selectors).unwrap()
    }

    fn get_thread_url(tr: &ElementRef) -> Option<String> {
        Some(
            tr.select(&title_selector())
                .next()?
                .attr("href")?
                .to_owned(),
        )
    }

    fn get_thread_title(tr: &ElementRef) -> Option<String> {
        Some(
            tr.select(&title_selector())
                .next()?
                .text()
                .next()?
                .trim()
                .to_string(),
        )
    }

    fn get_unix_timestamp(post_markup: &str) -> Option<DateTime<Utc>> {
        let document = Html::parse_document(post_markup);
        let selector = create_selector(".topic-body time");
        let datetime_str = document.select(&selector).next()?.attr("datetime")?;
        let datetime: DateTime<Utc> = datetime_str.parse().ok()?;
        Some(datetime)
    }
}
