use chrono::{DateTime, Utc};
use serde::Deserialize;
use thiserror::Error;

const POSTS_URL: &str = "https://us.forums.blizzard.com/en/d4/groups/blizzard-tracker/posts.json";
const BASE_URL: &str = "https://us.forums.blizzard.com/en/d4";

#[derive(Debug, Error)]
pub enum Error {
    #[error("http error: {0}")]
    Http(#[from] http::Error),
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiabloPost {
    #[serde(rename = "topic_title")]
    pub title: String,
    pub id: u32,
    #[serde(rename = "excerpt")]
    pub description: String,
    pub url: String,
    #[serde(rename = "created_at")]
    pub pub_date: DateTime<Utc>,
}

pub async fn fetch_posts() -> Result<Vec<DiabloPost>, Error> {
    let content = http::text(POSTS_URL).await?;
    let posts = parse_posts(&content)?;
    Ok(posts)
}

pub fn parse_posts(content: &str) -> Result<Vec<DiabloPost>, serde_json::Error> {
    #[derive(Debug, Clone, Deserialize)]
    struct RawDiabloPost {
        #[serde(rename = "topic_title")]
        pub title: String,
        pub id: u32,
        #[serde(rename = "excerpt")]
        pub description: String,
        #[serde(rename = "url")]
        pub pathname: String,
        #[serde(rename = "created_at")]
        pub pub_date: DateTime<Utc>,
    }

    #[derive(Debug, Clone, Deserialize)]
    struct Response {
        posts: Vec<RawDiabloPost>,
    }

    let posts = serde_json::from_str::<Response>(content)?.posts;
    let posts = posts
        .into_iter()
        .map(|raw_post| DiabloPost {
            title: raw_post.title,
            id: raw_post.id,
            description: html_escape::decode_html_entities(&raw_post.description).to_string(),
            url: format!("{}{}", BASE_URL, raw_post.pathname),
            pub_date: raw_post.pub_date,
        })
        .collect();

    Ok(posts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Timelike};

    #[tokio::test]
    async fn test_fetch_posts() {
        let posts = fetch_posts().await.unwrap();
        assert!(!posts.is_empty());
    }

    #[test]
    fn test_parse_from_fixture() {
        let content = include_str!("../tests/fixtures/posts.json");
        let posts = parse_posts(content).unwrap();
        assert_eq!(posts.len(), 20);

        let first_post = &posts[0];
        assert_eq!(first_post.id, 1989745);
        assert_eq!(
            first_post.url,
            "https://us.forums.blizzard.com/en/d4/t/embody-the-sector%E2%80%99s-finest-with-starcraft-x-diablo-iv/231450/1"
        );

        assert_eq!(
            first_post.pub_date.with_nanosecond(0).unwrap(),
            Utc.with_ymd_and_hms(2025, 9, 25, 17, 6, 22).unwrap()
        );

        assert_eq!(
            first_post.description,
            "<a href=\"https://bnetcmsus-a.akamaihd.net/cms/blog_header/47/47LPZ5UXDG1X1758584759888.png\">[Embody the Sector’s Finest with StarCraft x Diablo IV]</a> Faster than a Zerg rush, StarCraft storms into Sanctuary for a limited time.  <a href=\"https://news.blizzard.com/en-us/article/24224371\">View Full Article</a>"
        );
    }
}