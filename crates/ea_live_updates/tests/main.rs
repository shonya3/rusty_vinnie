use ea_live_updates::LiveUpdatesThread;

#[tokio::test]
async fn get_live_updates() {
    let updates = ea_live_updates::get_live_updates(LiveUpdatesThread::Ru)
        .await
        .unwrap();
    println!("{updates:#?}");

    // ea_live_updates::get_live_updates(live_updates_thread)
}
