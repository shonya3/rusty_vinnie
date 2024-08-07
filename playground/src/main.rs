#![allow(unused)]
use chrono::{
    DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Offset, TimeZone, Utc,
};
use fresh_news::{
    fetch_forum_threads, get_fresh_threads, NewsThreadInfo, Subforum, WebsiteLanguage,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[tokio::main]
async fn main() {}
