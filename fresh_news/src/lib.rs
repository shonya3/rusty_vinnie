use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{path::PathBuf, sync::Arc};

pub async fn get_fresh_news_url(lang: WebsiteLanguage) -> Result<Option<FreshNewsUrl>, Error> {
    FreshNewsUrl::get(lang).await
}

pub async fn get_fresh_threads(
    not_older_than_minutes: i64,
    lang: &WebsiteLanguage,
    subforum: &Subforum,
) -> Result<Vec<NewsThreadInfo>, Error> {
    NewsThreadInfo::get(not_older_than_minutes, lang, subforum).await
}

pub enum Subforum {
    News,
    PatchNotes,
}

impl std::fmt::Display for Subforum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subforum::News => f.write_str("news"),
            Subforum::PatchNotes => f.write_str("patch-notes"),
        }
    }
}

pub enum WebsiteLanguage {
    Ru,
    En,
}

impl std::fmt::Display for WebsiteLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebsiteLanguage::Ru => f.write_str("ru"),
            WebsiteLanguage::En => f.write_str("en"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct NewsThreadInfo {
    pub url: String,
    #[serde(rename = "postedDateISO")]
    pub posted_date: DateTime<Utc>,
    pub title: String,
}

impl NewsThreadInfo {
    pub const fn new(url: String, posted_date: DateTime<Utc>, title: String) -> Self {
        Self {
            url,
            posted_date,
            title,
        }
    }

    pub async fn get(
        not_older_than_minutes: i64,
        lang: &WebsiteLanguage,
        subforum: &Subforum,
    ) -> Result<Vec<Self>, Error> {
        let saved = Self::read_saved(lang, subforum)?;
        let fetched = Self::fetch(lang, subforum).await?;

        let actual: Vec<Self> = fetched
            .into_iter()
            .filter(|info| {
                info.age().num_minutes() <= not_older_than_minutes && !saved.contains(info)
            })
            .collect();

        Self::save(&actual, lang, subforum)?;

        Ok(actual)
    }

    pub fn age(&self) -> TimeDelta {
        Utc::now() - self.posted_date
    }

    pub fn path(lang: &WebsiteLanguage, subforum: &Subforum) -> Result<PathBuf, std::io::Error> {
        let dir = std::env::current_dir()?.join("data");
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        };
        let file_path = dir.join(format!("threadsInfo-{subforum}-{lang}.json"));
        if !file_path.exists() {
            std::fs::write(&file_path, json!([]).to_string())?;
        }
        Ok(file_path)
    }

    pub fn read_saved(
        lang: &WebsiteLanguage,
        subforum: &Subforum,
    ) -> Result<Vec<Self>, std::io::Error> {
        let path = Self::path(lang, subforum)?;
        let json = std::fs::read_to_string(path)?;
        if json.is_empty() {
            return Ok(vec![]);
        }
        Ok(serde_json::from_str(&json)?)
    }

    pub fn save(
        threads_info: &[Self],
        lang: &WebsiteLanguage,
        subforum: &Subforum,
    ) -> Result<(), Error> {
        let json = serde_json::to_string(&threads_info)?;
        Ok(std::fs::write(Self::path(lang, subforum)?, json)?)
    }

    pub async fn fetch(lang: &WebsiteLanguage, subforum: &Subforum) -> Result<Vec<Self>, Error> {
        let script = format!(
            "(el) => {{{} return getThreadsInfo()}}",
            include_str!("../getThreadsInfo.js")
        );

        let playwright = playwright::Playwright::initialize().await.unwrap();
        playwright.install_chromium().unwrap();
        let chrome = playwright.chromium();
        let browser = chrome.launcher().headless(true).launch().await?;
        let context = browser
            .context_builder()
            .clear_user_agent()
            .build()
            .await
            .unwrap();

        let announcements_url = match lang {
            WebsiteLanguage::Ru => {
                format!("https://ru.pathofexile.com/forum/view-forum/{subforum}")
            }
            WebsiteLanguage::En => {
                format!("https://www.pathofexile.com/forum/view-forum/{subforum}")
            }
        };

        let page = context.new_page().await.unwrap();
        page.goto_builder(&announcements_url).goto().await?;

        let threads_info: Vec<Self> = page.evaluate(&script, ()).await?;
        Ok(threads_info)
    }
}

#[derive(Debug, PartialEq)]
pub struct FreshNewsUrl(pub String);
impl FreshNewsUrl {
    pub const fn new(url: String) -> Self {
        Self(url)
    }

    pub async fn get(lang: WebsiteLanguage) -> Result<Option<Self>, Error> {
        let saved_url = Self::read_saved()?;
        let fetched_url = Self::fetch(lang).await?;

        match saved_url {
            Some(saved_url) => match saved_url == fetched_url {
                true => Ok(None),
                false => {
                    fetched_url.save()?;
                    Ok(Some(fetched_url))
                }
            },
            None => {
                fetched_url.save()?;
                Ok(Some(fetched_url))
            }
        }
    }

    fn path() -> Result<PathBuf, std::io::Error> {
        let dir = std::env::current_dir()?.join("data");
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        };

        Ok(dir.join("latest-news-url.txt"))
    }

    pub fn read_saved() -> Result<Option<Self>, std::io::Error> {
        let path = Self::path()?;

        Ok(std::fs::read_to_string(path).map(Self::new).ok())
    }

    pub fn save(&self) -> Result<(), Error> {
        Ok(std::fs::write(Self::path()?, &self.0)?)
    }

    pub async fn fetch(lang: WebsiteLanguage) -> Result<Self, Error> {
        let script = format!(
            "(el) => {{{} return getFreshNewsUrl()}}",
            include_str!("../getFreshNewsUrl.js")
        );

        let playwright = playwright::Playwright::initialize().await.unwrap();
        playwright.install_chromium().unwrap();
        let chrome = playwright.chromium();
        let browser = chrome.launcher().headless(true).launch().await?;
        let context = browser
            .context_builder()
            .clear_user_agent()
            .build()
            .await
            .unwrap();

        let announcements_url = match lang {
            WebsiteLanguage::Ru => "https://ru.pathofexile.com/forum/view-forum/news",
            WebsiteLanguage::En => "https://www.pathofexile.com/forum/view-forum/news",
        };

        let page = context.new_page().await.unwrap();
        page.goto_builder(announcements_url).goto().await?;

        let latest_url: String = page.evaluate(&script, ()).await?;

        Ok(Self::new(latest_url))
    }
}

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    Io(std::io::Error),
    Playwright(Arc<playwright::Error>),
    DateParse(chrono::ParseError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<Arc<playwright::Error>> for Error {
    fn from(value: Arc<playwright::Error>) -> Self {
        Self::Playwright(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(value: chrono::ParseError) -> Self {
        Self::DateParse(value)
    }
}
