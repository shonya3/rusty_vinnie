use std::time::Duration;

use playwright::Playwright;

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

/// Connects to Chrome via CDP and extracts challenge progress from PoE profile pages.
pub struct ChallengeExtractor {
    page: playwright::api::Page,
}

impl ChallengeExtractor {
    /// Creates a new extractor, connects to Chrome via CDP and navigates to challenges page.
    pub async fn new(playwright: &Playwright) -> Result<Self, playwright::Error> {
        let browser = playwright
            .chromium()
            .connect_over_cdp_builder("http://localhost:9222")
            .connect_over_cdp()
            .await?;

        let contexts = browser.contexts()?;
        let context = contexts.first().unwrap();
        let pages = context.pages()?;

        let page = if pages.is_empty() {
            context.new_page().await?
        } else {
            pages.first().unwrap().to_owned()
        };

        let extractor = Self { page };
        extractor.navigate().await?;

        Ok(extractor)
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

pub async fn run() {
    let playwright = Playwright::initialize().await.unwrap();
    playwright.install_chromium().unwrap();

    let mut extractor = ChallengeExtractor::new(&playwright).await.unwrap();
    println!("Starting extraction loop...\n");

    loop {
        match extractor.content().await {
            Ok(content) => {
                if let Some(remaining) = extractor.remaining(&content) {
                    let now = chrono::Local::now().format("%d.%m %H:%M");
                    let line = format!("{}: {}", now, remaining);
                    println!("{}", line);
                    std::fs::write(OUTPUT_FILE, line).ok();
                }
            }
            Err(e) => {
                println!("Failed to get content: {:?}", e);
                println!("Reconnecting...");
                extractor = ChallengeExtractor::new(&playwright).await.unwrap();
                
                // Fetch immediately after reconnect
                match extractor.content().await {
                    Ok(content) => {
                        if let Some(remaining) = extractor.remaining(&content) {
                            let now = chrono::Local::now().format("%d.%m %H:%M");
                            let line = format!("{}: {}", now, remaining);
                            println!("{}", line);
                            std::fs::write(OUTPUT_FILE, line).ok();
                        }
                    }
                    Err(e) => println!("Failed to reconnect: {:?}", e),
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(300)).await;

        if extractor.refresh().await.is_err() {
            println!("Page lost, reconnecting...");
            if extractor.navigate().await.is_err() {
                extractor = ChallengeExtractor::new(&playwright).await.unwrap();
            }
        }
    }
}
