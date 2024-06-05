// use fresh_news::NewsThreadInfo;

use fresh_news::{fetch_forum_threads, Subforum, WebsiteLanguage};

#[tokio::main]
async fn main() {
    let vec = fetch_forum_threads(&WebsiteLanguage::En, &Subforum::PatchNotes).await;
    println!("{vec:#?}");
}
