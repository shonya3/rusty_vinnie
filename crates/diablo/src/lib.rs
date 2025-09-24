use chrono::{DateTime, FixedOffset, Utc};
use thiserror::Error;

const URL: &str = "https://www.wowhead.com/diablo-4/blue-tracker?rss";

#[derive(Debug, Error)]
pub enum ItemParseError {
    #[error("missing field: {0}")]
    MissingField(String),
    #[error("chrono parse error: {0}")]
    Chrono(#[from] chrono::ParseError),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("http error: {0}")]
    Http(#[from] http::Error),
    #[error("rss parse error: {0}")]
    Rss(#[from] rss::Error),
    #[error("item parse error on item {1:?}: {0}")]
    ItemParse(#[source] ItemParseError, rss::Item),
}

#[derive(Debug, Clone)]
pub struct DiabloPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: DateTime<Utc>,
}

pub async fn fetch_posts() -> Result<Vec<DiabloPost>, Error> {
    let content = http::text(URL).await?;
    parse_posts(&content)
}

fn parse_item(item: &rss::Item) -> Result<DiabloPost, ItemParseError> {
    let title = item.title().unwrap_or_default().to_string();
    let link = item
        .link()
        .ok_or_else(|| ItemParseError::MissingField("link".to_string()))?
        .to_string();
    let description = item.description().unwrap_or_default().to_string();
    let pub_date = item
        .pub_date()
        .ok_or_else(|| ItemParseError::MissingField("pub_date".to_string()))?;
    let pub_date = DateTime::parse_from_rfc2822(pub_date)?.with_timezone(&Utc);

    Ok(DiabloPost {
        title,
        link,
        description,
        pub_date,
    })
}

pub fn parse_posts(content: &str) -> Result<Vec<DiabloPost>, Error> {
    let channel = rss::Channel::read_from(content.as_bytes())?;
    let posts = channel
        .into_items()
        .into_iter()
        .map(|item| parse_item(&item).map_err(|e| Error::ItemParse(e, item.clone())))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(posts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_posts() {
        let posts = fetch_posts().await.unwrap();
        assert!(!posts.is_empty());
    }

    #[test]
    fn test_parse_from_fixture() {
        let content = include_str!("../tests/fixtures/diablo_rss.xml");
        let posts = parse_posts(content).unwrap();
        assert_eq!(posts.len(), 1);

        let first_post = &posts[0];
        assert_eq!(first_post.title, "A title");
        assert_eq!(
            first_post.link,
            "https://www.wowhead.com/diablo-4/blue-tracker/topic/us/231178"
        );
    }
}
