use scraper::Html;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostDetails {
    pub content: String,
    pub image_src: Option<String>,
}

pub fn get_post_details(html: &str) -> Option<PostDetails> {
    let document = Html::parse_document(html);

    let el_content = selectors::content(&document)?;

    Some(PostDetails {
        content: markdown::html_to_markdown(&el_content),
        image_src: selectors::post_image_src(&document),
    })
}

mod selectors {
    use scraper::{ElementRef, Html, Selector};

    pub fn create_selector(selectors: &str) -> Selector {
        Selector::parse(selectors).unwrap()
    }

    fn content_post(document: &Html) -> Option<ElementRef<'_>> {
        document
            .select(&create_selector("tr.staff"))
            .next()
            .or_else(|| document.select(&create_selector(".newsPost")).next())
    }

    pub fn post_image_src(document: &Html) -> Option<String> {
        let el_content = content(document)?;
        el_content
            .select(&create_selector("img"))
            .next()?
            .attr("src")
            .map(|src| src.to_string())
    }

    pub fn content(document: &Html) -> Option<ElementRef<'_>> {
        content_post(document).and_then(|post| post.select(&create_selector(".content")).next())
    }

    #[allow(unused)]
    pub fn author(document: &Html) -> Option<String> {
        content_post(document).and_then(|post| {
            post.select(&create_selector(".post_by_account a"))
                .next()
                .map(|profile_link| profile_link.text().collect::<String>())
        })
    }
}
