use chrono::{DateTime, FixedOffset};

const URL: &str = "https://www.wowhead.com/diablo-4/blue-tracker?rss";

#[derive(Debug)]
pub struct DiabloPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: DateTime<FixedOffset>,
}

pub async fn fetch_posts() -> Result<Vec<DiabloPost>, Box<dyn std::error::Error>> {
    let content = http::text(URL).await?;
    parse_posts(&content)
}

pub fn parse_posts(content: &str) -> Result<Vec<DiabloPost>, Box<dyn std::error::Error>> {
    let channel = rss::Channel::read_from(content.as_bytes())?;
    let posts = channel
        .into_items()
        .into_iter()
        .map(|item| {
            let title = item.title().unwrap_or_default().to_string();
            let link = item.link().unwrap_or_default().to_string();
            let description = item.description().unwrap_or_default().to_string();
            let pub_date = item
                .pub_date()
                .and_then(|date_str| DateTime::parse_from_rfc2822(date_str).ok())
                .unwrap();

            DiabloPost {
                title,
                link,
                description,
                pub_date,
            }
        })
        .collect();

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
