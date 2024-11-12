use teasers::TeasersForumThread;

#[tokio::test]
async fn load_3_25_teasers_thread() {
    // 3.25 teasers forum thread
    let vec = teasers::download_teasers_from_thread(TeasersForumThread::Poe1_3_25Russian)
        .await
        .unwrap();
    assert!(vec.len() == 14);
}
