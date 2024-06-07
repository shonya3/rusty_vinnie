#![allow(unused)]

use std::str::FromStr;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Offset, TimeZone, Utc};
use fresh_news::{fetch_forum_threads, NewsThreadInfo, Subforum, WebsiteLanguage};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {}
