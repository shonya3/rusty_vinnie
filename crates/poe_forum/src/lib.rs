use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
const USER_AGENT: &str = "rusty_vinnie/0.1 (contact: poeshonya3@gmail.com)";

pub async fn fetch_subforum_threads_list(
    lang: WebsiteLanguage,
    subforum: Subforum,
    time_offset: Option<&FixedOffset>,
) -> Result<Vec<NewsThreadInfo>, reqwest::Error> {
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
    Ok(html::parse(&html, lang, time_offset))
}

#[derive(Debug, Clone, Copy)]
pub enum Subforum {
    News,
    PatchNotes,
    EarlyAccessPatchNotesEn,
    EarlyAccessPatchNotesRu,
    EarlyAccessAnnouncementsEn,
    EarlyAccessAnnouncementsRu,
}

impl std::fmt::Display for Subforum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subforum::News => f.write_str("news"),
            Subforum::PatchNotes => f.write_str("patch-notes"),
            Subforum::EarlyAccessPatchNotesEn => f.write_str("2212"),
            Subforum::EarlyAccessPatchNotesRu => f.write_str("2272"),
            Subforum::EarlyAccessAnnouncementsEn => f.write_str("2211"),
            Subforum::EarlyAccessAnnouncementsRu => f.write_str("2271"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
    pub author: Option<String>,
}

mod html {
    use crate::{NewsThreadInfo, WebsiteLanguage};
    use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Offset, ParseError, TimeZone, Utc};
    use scraper::{ElementRef, Html, Selector};

    pub fn parse(
        html: &str,
        lang: WebsiteLanguage,
        time_offset: Option<&FixedOffset>,
    ) -> Vec<NewsThreadInfo> {
        Html::parse_document(html)
            .select(&Selector::parse("table tbody tr").unwrap())
            .filter_map(|row| parse_tr(&row, lang, time_offset))
            .collect()
    }

    pub fn parse_tr(
        tr: &ElementRef,
        lang: WebsiteLanguage,
        time_offset: Option<&FixedOffset>,
    ) -> Option<NewsThreadInfo> {
        // Some(NewsThreadInfo::new(
        //     get_thread_url(tr, lang)?,
        //     get_posted_date(tr, lang, time_offset)?,
        //     get_thread_title(tr)?,
        // ))

        Some(NewsThreadInfo {
            url: get_thread_url(tr, lang)?,
            posted_date: get_posted_date(tr, lang, time_offset)?,
            title: get_thread_title(tr)?,
            author: get_author(tr),
        })
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

    fn get_thread_url(tr: &ElementRef, lang: WebsiteLanguage) -> Option<String> {
        let a_selector = &Selector::parse(".title a").ok()?;
        let path = tr.select(a_selector).next()?.attr("href")?.to_owned();
        let subdomain = match lang {
            WebsiteLanguage::Ru => "ru.",
            WebsiteLanguage::En => "www.",
        };
        Some(format!("https://{subdomain}pathofexile.com{path}"))
    }

    fn get_posted_date(
        tr: &ElementRef,
        lang: WebsiteLanguage,
        time_offset: Option<&FixedOffset>,
    ) -> Option<DateTime<Utc>> {
        let date_str = tr
            .select(&Selector::parse(".post_date").ok()?)
            .next()?
            .text()
            .next()?;

        match parse_forum_date(lang, date_str, time_offset) {
            Ok(date) => Some(date),
            Err(e) => {
                dbg!("Could not parse date. ", e, format!("{lang}"), date_str);
                None
            }
        }
    }

    fn get_author(tr: &ElementRef) -> Option<String> {
        let author = tr
            .select(&Selector::parse(".post_by_account a").unwrap())
            .next()?
            .text()
            .collect::<String>();
        Some(author)
    }

    fn parse_forum_date(
        lang: WebsiteLanguage,
        date_str: &str,
        time_offset: Option<&FixedOffset>,
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
                    "мар.",
                    "апр.",
                    "мая",
                    "июн.",
                    "июл.",
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

        let naive = NaiveDateTime::parse_from_str(&s, fmt)?;

        let local_date_time = match time_offset {
            Some(offset) => offset.from_local_datetime(&naive).unwrap(),
            None => Local::now()
                .offset()
                .fix()
                .from_local_datetime(&naive)
                .unwrap(),
        };

        Ok(local_date_time.to_utc())
    }
}
