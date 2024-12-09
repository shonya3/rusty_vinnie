use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const USER_AGENT: &str = "rusty_vinnie/0.1 (contact: poeshonya3@gmail.com)";

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(transparent)]
// pub struct LiveUpdateHeading(pub String);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct EarlyAccessDay(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LiveUpdate {
    pub day: EarlyAccessDay,
    pub heading: String,
    pub content: String,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct LiveUpdates(pub HashMap<EarlyAccessDay, Vec<LiveUpdate>>);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Hash, Eq)]
pub enum LiveUpdatesThread {
    Ru,
    En,
}

impl LiveUpdatesThread {
    pub fn url(&self) -> &'static str {
        match self {
            LiveUpdatesThread::Ru => "https://ru.pathofexile.com/forum/view-thread/3594084",
            LiveUpdatesThread::En => "https://www.pathofexile.com/forum/view-thread/3594080",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            LiveUpdatesThread::Ru => {
                "Запуск Path of Exile 2 в ранний доступ - Обновления в реальном времени🔴"
            }
            LiveUpdatesThread::En => "Path of Exile 2 Early Access Launch - Live Updates 🔴",
        }
    }
}

pub enum GetLiveUpdatesError {
    ParseMarkup(ParseMarkupError),
    Reqwest(reqwest::Error),
}

impl From<ParseMarkupError> for GetLiveUpdatesError {
    fn from(value: ParseMarkupError) -> Self {
        GetLiveUpdatesError::ParseMarkup(value)
    }
}

impl From<reqwest::Error> for GetLiveUpdatesError {
    fn from(value: reqwest::Error) -> Self {
        GetLiveUpdatesError::Reqwest(value)
    }
}

// "https://www.pathofexile.com/forum/view-thread/3594080"
pub async fn get_live_updates(
    live_updates_thread: LiveUpdatesThread,
) -> Result<Vec<LiveUpdate>, GetLiveUpdatesError> {
    let markup = reqwest::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()?
        .get(live_updates_thread.url())
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(parse_live_updates_thread(&markup, live_updates_thread)?)
}

#[derive(Debug)]
pub enum ParseMarkupError {}

#[allow(unused)]
pub fn parse_live_updates_thread(
    markup: &str,
    live_updates_thread: LiveUpdatesThread,
) -> Result<Vec<LiveUpdate>, ParseMarkupError> {
    todo!()
}
