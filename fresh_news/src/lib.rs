use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};

pub async fn get_fresh_news_url(lang: WebsiteLanguage) -> Result<Option<FreshNewsUrl>, Error> {
    FreshNewsUrl::get(lang).await
}

pub enum WebsiteLanguage {
    Ru,
    En,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct NewsThreadInfo {
    url: String,
    #[serde(rename = "postedDateISO")]
    posted_date: DateTime<Utc>,
}

impl NewsThreadInfo {
    pub const fn new(url: String, posted_date: DateTime<Utc>) -> Self {
        Self { url, posted_date }
    }

    // pub async fn get(
    //     not_older_than_minutes: u32,
    //     lang: WebsiteLanguage,
    // ) -> Result<Vec<Self>, Error> {
    //     let saved = Self::read_saved()?;
    //     let fetched = Self::fetch(lang).await?;

    //     let fetched: Vec<Self> = fetched
    //         .into_iter()
    //         .filter(|info| info.posted_minutes_ago < not_older_than_minutes)
    //         .collect();

    //     todo!()
    // }

    pub fn path() -> Result<PathBuf, std::io::Error> {
        let dir = std::env::current_dir()?.join("data");
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        };

        Ok(dir.join("threadsInfo.json"))
    }

    pub fn read_saved() -> Result<Vec<Self>, std::io::Error> {
        let path = Self::path()?;
        let json = std::fs::read_to_string(path)?;
        if json.is_empty() {
            return Ok(vec![]);
        }
        Ok(serde_json::from_str(&json)?)
    }

    pub fn save(threads_info: &[Self]) -> Result<(), Error> {
        let json = serde_json::to_string(&threads_info)?;
        Ok(std::fs::write(Self::path()?, json)?)
    }

    pub async fn fetch(lang: WebsiteLanguage) -> Result<Vec<Self>, Error> {
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
            WebsiteLanguage::Ru => "https://ru.pathofexile.com/forum/view-forum/news",
            WebsiteLanguage::En => "https://www.pathofexile.com/forum/view-forum/news",
        };

        let page = context.new_page().await.unwrap();
        page.goto_builder(announcements_url).goto().await?;

        #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
        #[serde(rename_all = "camelCase")]
        pub struct JSNewsThreadInfo {
            url: String,
            #[serde(rename = "postedDateISO")]
            posted_date: String,
        }

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

        Ok(std::fs::read_to_string(path).map(|url| Self::new(url)).ok())
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
