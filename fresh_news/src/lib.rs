use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
const USER_AGENT: &str = "rusty_vinnie/0.1 (contact: poeshonya3@gmail.com)";

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
        let fetched = fetch_forum_threads(lang, subforum).await?;

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
}

pub async fn fetch_forum_threads(
    lang: &WebsiteLanguage,
    subforum: &Subforum,
) -> Result<Vec<NewsThreadInfo>, Error> {
    let url = match lang {
        WebsiteLanguage::Ru => {
            format!("https://ru.pathofexile.com/forum/view-forum/{subforum}")
        }
        WebsiteLanguage::En => {
            format!("https://www.pathofexile.com/forum/view-forum/{subforum}")
        }
    };
    let client = reqwest::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()?;
    let html = client.get(url).send().await?.text().await?;
    Ok(html::parse(&html, lang))
}

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    Io(std::io::Error),
    DateParse(chrono::ParseError),
    ReqwestError(reqwest::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
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

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

mod html {
    use crate::{NewsThreadInfo, WebsiteLanguage};
    use chrono::{DateTime, NaiveDateTime, ParseError, Utc};
    use scraper::{ElementRef, Html, Selector};

    pub fn parse(html: &str, lang: &WebsiteLanguage) -> Vec<NewsThreadInfo> {
        Html::parse_document(html)
            .select(&Selector::parse("table tbody tr").unwrap())
            .filter_map(|row| parse_tr(&row, lang))
            .collect()
    }

    pub fn parse_tr(tr: &ElementRef, lang: &WebsiteLanguage) -> Option<NewsThreadInfo> {
        Some(NewsThreadInfo::new(
            get_thread_url(tr, lang)?,
            get_posted_date(tr, lang)?,
            get_thread_title(tr)?,
        ))
    }

    fn get_thread_title(tr: &ElementRef) -> Option<String> {
        let a_selector = &Selector::parse(".title a").ok()?;
        Some(
            tr.select(a_selector)
                .next()?
                .text()
                .next()?
                .trim()
                .to_string(),
        )
    }

    fn get_thread_url(tr: &ElementRef, lang: &WebsiteLanguage) -> Option<String> {
        let a_selector = &Selector::parse(".title a").ok()?;
        let path = tr.select(a_selector).next()?.attr("href")?.to_owned();
        let subdomain = match lang {
            WebsiteLanguage::Ru => "ru.",
            WebsiteLanguage::En => "www.",
        };
        Some(format!("{subdomain}pathofexile.com{path}"))
    }

    fn get_posted_date(tr: &ElementRef, lang: &WebsiteLanguage) -> Option<DateTime<Utc>> {
        let date_str = tr
            .select(&Selector::parse(".post_date").ok()?)
            .next()?
            .text()
            .next()?;

        match parse_forum_date(lang, date_str) {
            Ok(date) => Some(date),
            Err(e) => {
                eprintln!("Could not parse date {e}");
                None
            }
        }
    }

    fn parse_forum_date(
        lang: &WebsiteLanguage,
        date_str: &str,
    ) -> Result<DateTime<Utc>, ParseError> {
        let fmt = match lang {
            WebsiteLanguage::En => "%b %e, %Y, %I:%M:%S %p", // May 8, 2024, 4:37:26 PM
            WebsiteLanguage::Ru => "%d %m %Y, %H:%M:%S",     // 26 марта 2024 г., 5:10:44
        };
        let mut s = match lang {
            WebsiteLanguage::En => date_str.to_owned(),
            WebsiteLanguage::Ru => {
                let mut s = date_str.to_owned();
                for (index, month) in [
                    "янв.",
                    "февр.",
                    "марта",
                    "апр.",
                    "мая",
                    "июня",
                    "июля",
                    "авг.",
                    "сент.",
                    "окт.",
                    "нояб.",
                    "дек.",
                ]
                .iter()
                .enumerate()
                {
                    s = s.replace(month, &format!("{}", index + 1));
                }
                s.replace(" г.", "")
            }
        };
        if s.starts_with(", ") {
            s = s.chars().skip(2).collect();
        }

        NaiveDateTime::parse_from_str(&s, fmt).map(|naive| naive.and_utc())
    }
}
