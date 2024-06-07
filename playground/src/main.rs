#![allow(unused)]

use std::str::FromStr;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Offset, TimeZone, Utc};
use fresh_news::{fetch_forum_threads, NewsThreadInfo, Subforum, WebsiteLanguage};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let naive = NaiveDate::from_ymd_opt(2024, 6, 7)
        .unwrap()
        .and_hms_opt(4, 5, 0)
        .unwrap();
    let offset = Local::now().offset().fix();
    let local = offset.from_local_datetime(&naive).unwrap();
    let utc = local.to_utc();
    let utc2 = local.with_timezone(&Utc);
    println!("{}", utc == utc2);
}
