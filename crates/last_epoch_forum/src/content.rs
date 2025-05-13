use scraper::{Html, Selector};

pub fn create_selector(selectors: &str) -> Selector {
    Selector::parse(selectors).unwrap()
}

pub fn get_content(document: &Html) -> Option<String> {
    let el_content = document.select(&create_selector(".post")).next()?;

    Some(markdown::html_to_markdown(&el_content))
}
