use last_epoch_news::{NewsThreadInfo, Subforum};

#[tokio::test]
async fn prepares_threads_info_based_on_markup() {
    let html = std::fs::read_to_string("./tests/fixtures/announcements.html").unwrap();
    let threads = last_epoch_news::html::prepare_threads_info(&html).await;

    assert_eq!(3, threads.len());

    let a = NewsThreadInfo {
        url: "https://forum.lastepoch.com/t/last-epoch-season-2-tombs-of-the-erased-is-officially-live/75431".to_owned(),
        title: "Last Epoch - Season 2 - Tombs of the Erased is officially LIVE!".to_owned(),
        unix_timestamp: "2025-04-17T16:01:00Z".parse().unwrap(),
    };

    let b = NewsThreadInfo {
        url: "https://forum.lastepoch.com/t/supporter-packs-now-available/75419".to_owned(),
        title: "Supporter Packs now Available!".to_owned(),
        unix_timestamp: "2025-04-16T18:40:00Z".parse().unwrap(),
    };

    let c = NewsThreadInfo {
        url: "https://forum.lastepoch.com/t/48-hours-till-go-live/75386".to_owned(),
        title: "48 Hours till go live".to_owned(),
        unix_timestamp: "2025-04-15T16:02:00Z".parse().unwrap(),
    };

    assert!([a, b, c].iter().any(|t| threads.contains(t)));
}

#[tokio::test]
async fn fetches() {
    let result = last_epoch_news::fetch_subforum_threads_list(Subforum::Announcements).await;

    assert!(result.is_ok());
}
