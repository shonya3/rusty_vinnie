use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod content;

pub async fn fetch_subforum_threads_list(
    subforum: Subforum,
) -> Result<Vec<NewsThreadInfo>, reqwest::Error> {
    let html = http::text(&format!("https://forum.lastepoch.com/c/{subforum}")).await?;
    Ok(html::prepare_threads_info(&html, subforum).await)
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct NewsThreadInfo {
    pub url: String,
    pub title: String,
    pub datetime: DateTime<Utc>,
    pub content: Option<String>,
    pub author: Option<String>,
    pub subforum: Subforum,
    pub is_pinned: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
    use crate::{NewsThreadInfo, Subforum};
    use chrono::{DateTime, Utc};
    use scraper::{ElementRef, Html, Selector};

    const TITLE_SELECTOR: &str = "a.title";
    pub const TABLE_ROW_SELECTOR: &str = "table tbody tr";

    pub async fn prepare_threads_info(
        subforum_threads_page_html: &str,
        subforum: Subforum,
    ) -> Vec<NewsThreadInfo> {
        let url_title_ispinned = {
            let document = Html::parse_document(subforum_threads_page_html);
            let tr_selector = create_selector("table tbody tr");
            let is_pinneds = is_pinned::read_html(&document);

            document
                .select(&tr_selector)
                .take(3)
                .enumerate()
                .filter_map(|(index, row)| {
                    Some((
                        get_thread_url(&row)?.to_owned(),
                        get_thread_title(&row)?.to_owned(),
                        is_pinneds
                            .as_ref()
                            .and_then(|is_pinneds| is_pinneds.get(index).copied())
                            .unwrap_or(false),
                    ))
                })
                .collect::<Vec<(String, String, bool)>>()
        };

        let mut results = Vec::new();
        for (url, title, is_pinned) in url_title_ispinned.into_iter() {
            if let Ok(post_page_html) = http::text(&url).await {
                let document = Html::parse_document(&post_page_html);

                if let Some(datetime) = get_datetime(&document) {
                    results.push(NewsThreadInfo {
                        url,
                        title,
                        datetime,
                        content: crate::content::get_content(&document),
                        author: get_author(&document),
                        subforum,
                        is_pinned,
                    });
                }
            }
        }

        results
    }

    fn get_author(document: &Html) -> Option<String> {
        document
            .select(&create_selector(".creator"))
            .next()
            .map(|creator| creator.text().collect::<String>())
    }

    pub fn create_selector(selectors: &str) -> Selector {
        Selector::parse(selectors).unwrap()
    }

    fn get_thread_url(tr: &ElementRef) -> Option<String> {
        Some(
            tr.select(&create_selector(TITLE_SELECTOR))
                .next()?
                .attr("href")?
                .to_owned(),
        )
    }

    fn get_thread_title(tr: &ElementRef) -> Option<String> {
        Some(
            tr.select(&create_selector(TITLE_SELECTOR))
                .next()?
                .text()
                .next()?
                .trim()
                .to_string(),
        )
    }

    fn get_datetime(document: &Html) -> Option<DateTime<Utc>> {
        let selector = create_selector(".topic-body time");
        let datetime_str = document.select(&selector).next()?.attr("datetime")?;
        let datetime: DateTime<Utc> = datetime_str.parse().ok()?;
        Some(datetime)
    }

    /// Check if thread is pinned at the top of thread subforum thread list.
    mod is_pinned {
        use chrono::NaiveDate;
        use scraper::Html;

        use super::create_selector;
        use crate::html::TABLE_ROW_SELECTOR;

        /// Returns array of is_pinned bools from given html of subforum threads list.
        pub fn read_html(document: &Html) -> Option<Vec<bool>> {
            let naive_dates = extract_naive_dates_from_subforum_threads_list(document).unwrap();
            Some(analyze_dates(naive_dates))
        }

        fn extract_naive_dates_from_subforum_threads_list(
            document: &Html,
        ) -> Option<Vec<NaiveDate>> {
            let row_selector = crate::html::create_selector(TABLE_ROW_SELECTOR);
            let rows = document.select(&row_selector);

            let mut naive_dates = vec![];
            for row in rows.take(4) {
                let last_td = row.select(&create_selector("td")).last().unwrap();
                let date_str = last_td.text().collect::<String>();
                let date_str = date_str.trim();
                let date = get_rough_date(date_str).ok().unwrap();
                naive_dates.push(date);
            }

            Some(naive_dates)
        }

        fn get_rough_date(date_str: &str) -> chrono::ParseResult<NaiveDate> {
            NaiveDate::parse_from_str(date_str, "%B %d, %Y")
        }

        /// Returns array of is_pinned bools
        pub fn analyze_dates(naive_dates: Vec<NaiveDate>) -> Vec<bool> {
            let mut is_pinneds: Vec<bool> = vec![];
            let mut cur_index = naive_dates.len() - 1;

            loop {
                let cur = naive_dates[cur_index];
                let cur_minus_one = naive_dates[cur_index - 1];

                is_pinneds.push(false);

                cur_index -= 1;
                if cur > cur_minus_one || cur_index == 0 {
                    break;
                }
            }

            while is_pinneds.len() != naive_dates.len() {
                is_pinneds.push(true);
            }

            is_pinneds.reverse();

            is_pinneds
        }

        #[cfg(test)]
        mod is_pinned_tests {
            use chrono::NaiveDate;

            #[test]
            fn analyze_dates() {
                fn test_group(expected: Vec<bool>, date_strs: Vec<&str>) {
                    let parse_naive_date =
                        |date_str| NaiveDate::parse_from_str(date_str, "%B %d, %Y").unwrap();
                    let naive_dates = date_strs
                        .into_iter()
                        .map(parse_naive_date)
                        .collect::<Vec<_>>();

                    assert_eq!(expected, super::analyze_dates(naive_dates))
                }

                test_group(
                    vec![true, true, false, false],
                    vec![
                        "March 7, 2022",
                        "March 7, 2022",
                        "May 7, 2025",
                        "April 8, 2025",
                    ],
                );

                test_group(
                    vec![true, false, false],
                    vec!["August 26, 2021", "May 15, 2025", "May 14, 2025"],
                );
            }
        }
    }
}
