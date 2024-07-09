#![allow(unused)]

use std::str::FromStr;

use chrono::{
    DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Offset, TimeZone, Utc,
};
use fresh_news::{
    fetch_forum_threads, get_fresh_threads, NewsThreadInfo, Subforum, WebsiteLanguage,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let teas = teasers::download_teasers_from_thread(
        "https://ru.pathofexile.com/forum/view-thread/3530604/page/1",
    )
    .await
    .unwrap();
    println!("{teas:#?}");
}
