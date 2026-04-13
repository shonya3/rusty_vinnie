use playwright::{Playwright, api::Page};
use std::time::Duration;

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

const TARGET_TIERS: u32 = 8000;
const OUTPUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\..\\..\\remaining_tiers.txt");
const HISTORY_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\..\\..\\tiers_history.txt");

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

    // Read previous from file
    let mut previous_remaining: Option<u32> = std::fs::read_to_string(OUTPUT_FILE)
        .ok()
        .and_then(|c| c.split_whitespace().last()?.parse().ok());

    if let Some(prev) = previous_remaining {
        println!("Loaded previous remaining: {}", prev);
    }

    println!("Starting extraction loop...\n");

    loop {
        let content = extractor.content().await?;
        if let Some(remaining) = extractor.remaining(&content) {
            let now = chrono::Local::now().format("%d.%m %H:%M");
            let line = format!("{}: {}", now, remaining);
            println!("{}", line);
            std::fs::write(OUTPUT_FILE, line).ok();

            if previous_remaining != Some(remaining) {
                let history_line = format!("{}: {}\n", now, remaining);
                if let Ok(mut file) = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(HISTORY_FILE)
                {
                    use std::io::Write;
                    file.write_all(history_line.as_bytes()).ok();
                }
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
