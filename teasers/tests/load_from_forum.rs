#[tokio::test]
async fn load_3_25_teasers_thread() {
    // 3.25 teasers forum thread
    let url = "https://ru.pathofexile.com/forum/view-thread/3530604/page/1";
    let vec = teasers::download_teasers_from_thread(url).await.unwrap();
    assert!(vec.len() == 14);
}
