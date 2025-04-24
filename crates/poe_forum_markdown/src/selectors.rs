use scraper::{ElementRef, Selector};

pub fn create_selector(selectors: &str) -> Selector {
    Selector::parse(selectors).unwrap()
}

fn patchnotes_post<'a>(e: &'a ElementRef) -> Option<ElementRef<'a>> {
    e.select(&create_selector("tr.staff")).next()
}

pub fn content<'a>(e: &'a ElementRef) -> Option<ElementRef<'a>> {
    patchnotes_post(e).and_then(|post| post.select(&create_selector(".content")).next())
}

pub fn author(e: &ElementRef) -> Option<String> {
    patchnotes_post(e).and_then(|post| {
        post.select(&create_selector(".post_by_account a"))
            .next()
            .map(|profile_link| profile_link.text().collect::<String>())
    })
}
