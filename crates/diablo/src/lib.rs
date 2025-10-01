use chrono::{DateTime, Utc};
use regex::Regex;
use serde::Deserialize;
use serde_repr::Serialize_repr;
use thiserror::Error;

const POSTS_URL: &str = "https://us.forums.blizzard.com/en/d4/groups/blizzard-tracker/posts.json";
const BASE_URL: &str = "https://us.forums.blizzard.com/en/d4";
const WEBSITE_DOMAIN_URL: &str = "https://us.forums.blizzard.com";

#[derive(Debug, Error)]
pub enum Error {
    #[error("http error: {0}")]
    Http(#[from] http::Error),
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("regex error: {0}")]
    Regex(#[from] regex::Error),
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub avatar_url: String,
}

impl User {
    pub fn profile_url(&self) -> String {
        format!("{}/u/{}/activity", BASE_URL, self.name)
    }
}

#[derive(Debug, Clone)]
pub enum PostKind {
    News { post_image_url: Option<String> },
    Other,
}

/// Represents "category_id" number field
#[derive(Clone, Copy, Serialize_repr, Debug, PartialEq)]
#[repr(u16)]
pub enum PostCategory {
    PcGeneralDiscussion = 5,
    ConsoleDiscussion = 6,
    ConsoleBugReport = 11,
    Other = u16::MAX,
}

impl PostCategory {
    pub fn is_console_related(&self) -> bool {
        matches!(
            self,
            PostCategory::ConsoleDiscussion | PostCategory::ConsoleBugReport
        )
    }
}

impl<'de> Deserialize<'de> for PostCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u16::deserialize(deserializer)?;
        Ok(match value {
            5 => PostCategory::PcGeneralDiscussion,
            6 => PostCategory::ConsoleDiscussion,
            11 => PostCategory::ConsoleBugReport,
            _ => PostCategory::Other,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DiabloPost {
    pub title: String,
    pub id: u32,
    pub description: String,
    pub url: String,
    pub pub_date: DateTime<Utc>,
    pub user: User,
    pub kind: PostKind,
    pub category: PostCategory,
}

pub async fn fetch_posts() -> Result<Vec<DiabloPost>, Error> {
    let content = http::text(POSTS_URL).await?;
    let posts = parse_posts(&content)?;
    Ok(posts)
}

pub fn parse_posts(content: &str) -> Result<Vec<DiabloPost>, Error> {
    #[derive(Debug, Clone, Deserialize)]
    struct RawUser {
        pub id: u32,
        #[serde(rename = "username")]
        pub name: String,
        pub avatar_template: String,
    }

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
        pub user: RawUser,
        pub category_id: PostCategory,
    }

    #[derive(Debug, Clone, Deserialize)]
    struct Response {
        posts: Vec<RawDiabloPost>,
    }

    let posts = serde_json::from_str::<Response>(content)?.posts;
    let re = Regex::new(r##"<a href=\"([^\\"]+\.(?:png|jpg|jpeg|gif))\".*?>.*?</a>"##)?;

    let posts = posts
        .into_iter()
        .map(|raw_post| {
            let description = html_escape::decode_html_entities(&raw_post.description).to_string();
            let (kind, description) = if raw_post.user.id == 1 {
                if let Some(captures) = re.captures(&description) {
                    let image_url = captures.get(1).map(|m| m.as_str().to_string());
                    let description = re.replace(&description, "").to_string();
                    (
                        PostKind::News {
                            post_image_url: image_url,
                        },
                        description,
                    )
                } else {
                    (
                        PostKind::News {
                            post_image_url: None,
                        },
                        description,
                    )
                }
            } else {
                (PostKind::Other, description)
            };

            let avatar_url = {
                let avatar = &raw_post.user.avatar_template;
                match avatar.starts_with("https") {
                    true => avatar.clone(),
                    false => format!("{}{}", WEBSITE_DOMAIN_URL, avatar.replace("{size}", "128")),
                }
            };

            let user = User {
                id: raw_post.user.id,
                name: raw_post.user.name,
                avatar_url,
            };

            DiabloPost {
                title: raw_post.title,
                id: raw_post.id,
                description: html2md::parse_html(&description).trim().to_string(),
                url: format!("{}{}", BASE_URL, raw_post.pathname),
                pub_date: raw_post.pub_date,
                user,
                kind,
                category: raw_post.category_id,
            }
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
            "Faster than a Zerg rush, StarCraft storms into Sanctuary for a limited time. [View Full Article](https://news.blizzard.com/en-us/article/24224371)"
        );

        match &first_post.kind {
            PostKind::News { post_image_url } => {
                assert_eq!(post_image_url.as_deref(), Some("https://bnetcmsus-a.akamaihd.net/cms/blog_header/47/47LPZ5UXDG1X1758584759888.png"));
            }
            _ => panic!("Expected PostKind::News"),
        }

        assert_eq!(first_post.user.id, 1);
        assert_eq!(first_post.user.name, "BlizzardEntertainment");
        assert_eq!(
            first_post.user.profile_url(),
            "https://us.forums.blizzard.com/en/d4/u/BlizzardEntertainment/activity"
        );
        assert_eq!(first_post.category, PostCategory::PcGeneralDiscussion);
        assert_eq!(first_post.user.avatar_url, "https://us.forums.blizzard.com/en/d4/plugins/discourse-blizzard-plugin/images/avatars/d4/default.png");
    }
}
