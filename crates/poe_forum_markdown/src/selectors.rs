use scraper::{ElementRef, Html, Selector};

pub fn create_selector(selectors: &str) -> Selector {
    Selector::parse(selectors).unwrap()
}

fn content_post(document: &Html) -> Option<ElementRef> {
    document.select(&create_selector("tr.staff")).next()
}

pub fn content(document: &Html) -> Option<ElementRef> {
    content_post(document).and_then(|post| post.select(&create_selector(".content")).next())
}

pub fn author(document: &Html) -> Option<String> {
    content_post(document).and_then(|post| {
        post.select(&create_selector(".post_by_account a"))
            .next()
            .map(|profile_link| profile_link.text().collect::<String>())
    })
}
