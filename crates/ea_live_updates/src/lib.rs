use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct EarlyAccessDay(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LiveUpdate {
    pub day: u32,
    pub heading: String,
    pub content: String,
    pub thread: LiveUpdatesThread,
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
            LiveUpdatesThread::Ru => "https://ru.pathofexile.com/forum/view-thread/3741051",
            LiveUpdatesThread::En => "https://www.pathofexile.com/forum/view-thread/3741050",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            LiveUpdatesThread::Ru => {
                "Запуск Path of Exile 2: Начало охоты - Обновления в реальном времени 🔴"
            }
            LiveUpdatesThread::En => "Path of Exile 2: Dawn of the Hunt Launch - Live Updates 🔴",
        }
    }
}

pub async fn get_live_updates(
    live_updates_thread: LiveUpdatesThread,
) -> Result<Vec<LiveUpdate>, reqwest::Error> {
    let markup = http::text(live_updates_thread.url()).await?;
    Ok(parse_live_updates_thread(&markup, live_updates_thread))
}

#[allow(unused)]
pub fn parse_live_updates_thread(
    markup: &str,
    live_updates_thread: LiveUpdatesThread,
) -> Vec<LiveUpdate> {
    let sections = break_markup_into_day_sections(markup);
    let latest_day_section = sections.first().cloned().unwrap_or_default();
    println!("{latest_day_section}");
    parse_day_section(&latest_day_section, live_updates_thread)
}

pub fn break_markup_into_day_sections(markup: &str) -> Vec<String> {
    let s: String = markup
        .lines()
        .take_while(|line| !line.contains(r#"<tr class="newsPost newsPostInfo">"#))
        .collect();

    let indices = s.match_indices("<h3>").map(|(i, _)| i).collect::<Vec<_>>();
    let h3_sections = s
        .match_indices("<h3>")
        .enumerate()
        .map(|(index, (start, _))| {
            let end = indices.get(index + 1).copied().unwrap_or(s.len());

            s[start..end].to_owned()
        })
        .collect::<Vec<_>>();

    h3_sections
}

pub fn parse_day_section(input: &str, thread: LiveUpdatesThread) -> Vec<LiveUpdate> {
    // Updated regex to match both <br/> and <br /> variations
    let h3_re = Regex::new(r"<h3>.*?Access Day (\d+)</h3>").unwrap();
    let strong_re = Regex::new(r"<strong>(.*?)</strong> - (.*?)<br\s*/?>").unwrap();

    // Extract the day number from <h3>
    let day: u32 = h3_re
        .captures(input)
        .and_then(|cap| cap.get(1).and_then(|m| m.as_str().parse().ok()))
        .unwrap_or(0);

    // Extract all <strong> and their associated content
    let mut updates = Vec::new();
    for caps in strong_re.captures_iter(input) {
        let heading = caps
            .get(1)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let content = caps
            .get(2)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();

        updates.push(LiveUpdate {
            day,
            heading,
            content,
            thread,
        });
    }

    updates
}

#[cfg(test)]
mod tests {
    use crate::{parse_day_section, LiveUpdatesThread};

    #[test]
    fn parse() {
        let markup = std::fs::read_to_string("./tests/ea_updates.html").unwrap();
        let sections = super::break_markup_into_day_sections(&markup);
        assert_eq!(3, sections.len());
        assert!(sections[2].contains("<h3>Path of Exile 2 Early Access Day 1</h3>"));
    }

    #[test]
    fn day() {
        let markup = std::fs::read_to_string("./tests/ea_updates.html").unwrap();
        let sections = super::break_markup_into_day_sections(&markup);
        let thread = LiveUpdatesThread::En;
        assert_eq!(parse_day_section(&sections[0], thread).len(), 2);
        assert_eq!(parse_day_section(&sections[1], thread).len(), 2);
        assert_eq!(parse_day_section(&sections[2], thread).len(), 14);
    }
}
