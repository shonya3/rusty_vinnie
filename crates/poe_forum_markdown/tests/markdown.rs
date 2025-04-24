use poe_forum_markdown::markdown;
use scraper::{Html, Selector};

fn create_selector(selectors: &str) -> Selector {
    Selector::parse(selectors).unwrap()
}

#[test]
fn html_to_markdown() {
    let document = Html::parse_document(
        &std::fs::read_to_string("tests/fixtures/patch-notes-0.2.0e.html").unwrap(),
    );

    let expected = std::fs::read_to_string("tests/fixtures/patch-notes-0.2.0e.md").unwrap();

    let patch_notes_post = document
        .select(&create_selector("tr.staff"))
        .next()
        .unwrap();

    let el_content = patch_notes_post
        .select(&create_selector(".content"))
        .next()
        .unwrap();

    let markdown_content = markdown::html_to_markdown(&el_content);

    assert_eq!(expected, markdown_content);
}
