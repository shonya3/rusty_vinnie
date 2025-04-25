use crate::{content, selectors};
use scraper::Html;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostDetails {
    pub content: String,
    pub image_src: Option<String>,
}

pub fn get_details(html: &str) -> Option<PostDetails> {
    let document = Html::parse_document(html);

    let el_content = selectors::content(&document)?;

    Some(PostDetails {
        content: content::html_to_markdown(&el_content),
        image_src: selectors::post_image_src(&document),
    })
}
