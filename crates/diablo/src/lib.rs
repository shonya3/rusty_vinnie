use chrono::{DateTime, Utc};
use serde::Deserialize;
use thiserror::Error;

const URL: &str = "https://us.forums.blizzard.com/en/d4/groups/blizzard-tracker/posts.json";
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

impl DiabloPost {
    pub fn link(&self) -> String {
        format!("{}{}", BASE_URL, self.url)
    }
}

pub async fn fetch_posts() -> Result<Vec<DiabloPost>, Error> {
    let content = http::text(URL).await?;
    let posts = parse_posts(&content)?;
    Ok(posts)
}

pub fn parse_posts(content: &str) -> Result<Vec<DiabloPost>, serde_json::Error> {
    #[derive(Debug, Clone, Deserialize)]
    struct Response {
        posts: Vec<DiabloPost>,
    }

    let posts = serde_json::from_str::<Response>(content)?.posts;

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
        assert_eq!(first_post.id, 1989634);
        assert_eq!(
            first_post.link(),
            "https://us.forums.blizzard.com/en/d4/t/are-chaos-items-bugged/231417/2"
        );

        assert_eq!(
            first_post.pub_date.with_nanosecond(0).unwrap(),
            Utc.with_ymd_and_hms(2025, 9, 25, 14, 45, 46).unwrap()
        );
    }
}
