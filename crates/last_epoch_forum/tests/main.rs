use chrono::{DateTime, Utc};
use last_epoch_forum::{NewsThreadInfo, Subforum};
use scraper::Html;

#[tokio::test]
async fn prepares_threads_info_based_on_markup() {
    let html = std::fs::read_to_string("./tests/fixtures/announcements.html").unwrap();
    let threads =
        last_epoch_forum::html::prepare_threads_info(&html, Subforum::Announcements).await;

    assert_eq!(3, threads.len());

    #[derive(PartialEq)]
    struct PartialInfo {
        url: String,
        title: String,
        datetime: DateTime<Utc>,
    }

    impl From<NewsThreadInfo> for PartialInfo {
        fn from(value: NewsThreadInfo) -> Self {
            PartialInfo {
                url: value.url,
                title: value.title,
                datetime: value.datetime,
            }
        }
    }

    let partials = threads
        .into_iter()
        .map(PartialInfo::from)
        .collect::<Vec<_>>();

    let a = PartialInfo {
        url: "https://forum.lastepoch.com/t/last-epoch-season-2-tombs-of-the-erased-is-officially-live/75431".to_owned(),
        title: "Last Epoch - Season 2 - Tombs of the Erased is officially LIVE!".to_owned(),
        datetime: "2025-04-17T16:01:00Z".parse().unwrap(),
    };

    let b = PartialInfo {
        url: "https://forum.lastepoch.com/t/supporter-packs-now-available/75419".to_owned(),
        title: "Supporter Packs now Available!".to_owned(),
        datetime: "2025-04-16T18:40:00Z".parse().unwrap(),
    };

    let c = PartialInfo {
        url: "https://forum.lastepoch.com/t/48-hours-till-go-live/75386".to_owned(),
        title: "48 Hours till go live".to_owned(),
        datetime: "2025-04-15T16:02:00Z".parse().unwrap(),
    };

    assert!([a, b, c].iter().all(|t| partials.contains(t)));
}

#[tokio::test]
async fn fetches() {
    let result = last_epoch_forum::fetch_subforum_threads_list(Subforum::Announcements).await;

    assert!(result.is_ok());
}

#[test]
fn parses_body_markdown() {
    let html = std::fs::read_to_string("tests/fixtures/1.2.3/input.html").unwrap();
    let actual = last_epoch_forum::content::get_content(&Html::parse_document(&html));

    assert!(actual.is_some());

    let expected = std::fs::read_to_string("tests/fixtures/1.2.3/expected.md").unwrap();
    assert_eq!(expected, actual.unwrap());
}
