use chrono::{DateTime, FixedOffset, Utc};
use playwright::{Playwright, api::Page};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct TierEntry {
    pub datetime: DateTime<Utc>,
    pub remaining: u32,
}

impl std::fmt::Display for TierEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let moscow = self
            .datetime
            .with_timezone(&FixedOffset::east_opt(3 * 3600).unwrap());
        write!(f, "{}: {}", moscow.format("%d.%m %H:%M"), self.remaining)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct TiersHistory {
    pub entries: Vec<TierEntry>,
}

impl std::fmt::Display for TiersHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in &self.entries {
            writeln!(f, "{}", entry)?;
        }
        Ok(())
    }
}

pub fn load_history() -> TiersHistory {
    std::fs::read_to_string(paths::tiers_history())
        .ok()
        .and_then(|c| serde_json::from_str(&c).ok())
        .unwrap_or_default()
}

mod paths {
    use std::path::PathBuf;

    pub fn workspace_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }

    pub fn tiers_history() -> PathBuf {
        workspace_root().join("tiers_history.json")
    }
}

const TARGET_TIERS: u32 = 8000;

fn extract_current_tiers(content: &str) -> Option<u32> {
    if let Some(start) = content.find("Tyrannical Tiers") {
        let chunk = &content[start..start + 200.min(content.len() - start)];
        if let Some(span_start) = chunk.find(" completion-incomplete\">") {
            let after_span = &chunk[span_start + 24..];
            let num: String = after_span
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            return num.parse().ok();
        }
    }
    None
}

/// Connects to Chrome via CDP and extracts challenge progress from PoE profile pages.
pub struct ChallengeExtractor<'a> {
    page: &'a playwright::api::Page,
}

impl<'a> ChallengeExtractor<'a> {
    pub fn new(page: &'a Page) -> Self {
        Self { page }
    }

    /// Calculates remaining tiers to reach [`TARGET_TIERS`]
    pub fn remaining(&self, content: &str) -> Option<u32> {
        extract_current_tiers(content).map(|current| TARGET_TIERS - current)
    }

    /// Reloads the page.
    pub async fn refresh(&self) -> Result<(), playwright::Error> {
        self.page
            .reload_builder()
            .wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
            .reload()
            .await
            .map_err(playwright::Error::from)
            .map(|_| ())
    }

    /// Navigates to challenges page.
    pub async fn navigate(&self) -> Result<(), playwright::Error> {
        let url = "https://www.pathofexile.com/account/view-profile/Frxtl-5064/challenges";
        self.page
            .goto_builder(url)
            .wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
            .goto()
            .await
            .map_err(playwright::Error::from)
            .map(|_| ())
    }

    /// Returns the raw HTML content of the page.
    pub async fn content(&self) -> std::result::Result<String, playwright::Error> {
        self.page.content().await.map_err(playwright::Error::from)
    }
}

async fn connect_and_extract() -> Result<(), Box<dyn std::error::Error>> {
    let playwright = Playwright::initialize().await?;
    playwright.install_chromium()?;

    let browser = playwright
        .chromium()
        .connect_over_cdp_builder("http://localhost:9222")
        .connect_over_cdp()
        .await?;

    let contexts = browser.contexts()?;
    let context = contexts.first().ok_or("No browser context")?;
    let pages = context.pages()?;

    let page = if pages.is_empty() {
        context.new_page().await?
    } else {
        pages.first().unwrap().to_owned()
    };

    let extractor = ChallengeExtractor::new(&page);
    extractor.navigate().await?;

    let mut tiers_history = load_history();

    let mut previous_remaining = tiers_history.entries.last().map(|e| e.remaining);

    if let Some(prev) = previous_remaining {
        println!("Loaded previous remaining: {}", prev);
    }

    println!("Starting extraction loop...\n");

    loop {
        let content = extractor.content().await?;
        if let Some(remaining) = extractor.remaining(&content) {
            let entry = TierEntry {
                datetime: Utc::now(),
                remaining,
            };
            println!("{}", entry);

            if previous_remaining != Some(remaining) {
                tiers_history.entries.push(entry);
                std::fs::write(
                    paths::tiers_history(),
                    serde_json::to_string_pretty(&tiers_history).unwrap(),
                )
                .ok();
                previous_remaining = Some(remaining);
            }
        }

        tokio::time::sleep(Duration::from_secs(300)).await;
        extractor.refresh().await?;
    }
}

pub async fn run() {
    loop {
        if let Err(e) = connect_and_extract().await {
            println!("\nConnection lost: {}. Reconnecting in 5 seconds...\n", e);
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiers_history_display() {
let json = r#"{
            "entries": [
                {"datetime": "2025-04-11T15:37:00Z", "remaining": 1326},
                {"datetime": "2025-04-12T10:17:00Z", "remaining": 1262},
                {"datetime": "2025-04-12T17:45:00Z", "remaining": 1246},
                {"datetime": "2025-04-13T04:09:00Z", "remaining": 1230},
                {"datetime": "2025-04-13T18:55:00Z", "remaining": 1214}
            ]
        }"#;

        let history: TiersHistory = serde_json::from_str(json).unwrap();

        let expected = "11.04 18:37: 1326\n12.04 13:17: 1262\n12.04 20:45: 1246\n13.04 07:09: 1230\n13.04 21:55: 1214\n";

        assert_eq!(expected, history.to_string());
    }
}
