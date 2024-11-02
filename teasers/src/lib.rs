use error::Error;
use scraper::{selectable::Selectable, ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
const USER_AGENT: &str = "rusty_vinnie/0.1 (contact: poeshonya3@gmail.com)";

pub mod error;

pub async fn download_teasers_from_thread(url: &str) -> Result<Vec<Teaser>, Error> {
    let thread_markup = reqwest::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()?
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    Ok(parse_teasers_thread(&thread_markup)?)
}

pub fn parse_teasers_thread(markup: &str) -> Result<Vec<Teaser>, ParseTeasersThreadError> {
    let html = Html::parse_document(markup);
    let teasers_post = html
        .select(&Selector::parse("tr.newsPost").unwrap())
        .next()
        .ok_or(ParseTeasersThreadError::NoNewsPost)?;

    let spoiler_content_iframe_selector = Selector::parse(".spoilerContent iframe").unwrap();
    let spoiler_content_img_selector = Selector::parse(".spoilerContent img").unwrap();

    Ok(teasers_post
        .select(&Selector::parse("h2").unwrap())
        .filter_map(|h2| {
            let content = next_sibling_element(&h2).and_then(|spoiler_element| {
                match spoiler_element
                    .select(&spoiler_content_iframe_selector)
                    .next()
                {
                    Some(iframe) => iframe.attr("src").and_then(|src| match src {
                        src if src.starts_with("//www.youtube.com/") => {
                            Some(format!("https:{src}").replace("embed", "watch"))
                        }

                        src if src.starts_with("https://player.vimeo.com/video/") => {
                            Some(src.replace("player.vimeo.com/video/", "vimeo.com/"))
                        }

                        src if src.starts_with("https://") => Some(src.replace("embed", "watch")),

                        _ => None,
                    }),
                    None => spoiler_element
                        .select(&spoiler_content_img_selector)
                        .next()
                        .and_then(|img_el| img_el.attr("src"))
                        .map(|url| url.to_string()),
                }
            })?;

            Some(Teaser {
                heading: h2
                    .text()
                    .collect::<String>()
                    .trim()
                    .replace('\n', " ")
                    .replace('\t', ""),
                content,
            })
        })
        .collect())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Teaser {
    pub heading: String,
    pub content: String,
}

#[derive(Debug)]
pub enum ParseTeasersThreadError {
    NoNewsPost,
}

impl Display for ParseTeasersThreadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseTeasersThreadError::NoNewsPost => {
                f.write_str("Invalid thread. No news post found")
            }
        }
    }
}

fn next_sibling_element<'a>(element: &'a ElementRef) -> Option<ElementRef<'a>> {
    let mut sibling = element.next_sibling();
    while let Some(sib) = sibling {
        if let Some(element) = ElementRef::wrap(sib) {
            return Some(element);
        }
        sibling = sib.next_sibling();
    }

    None
}

#[cfg(test)]
mod tests {
    use scraper::{Html, Selector};

    #[test]
    fn next_sibling() {
        let markup = r#"<h2>Прибавки от качества на броне и оружии теперь мультипликативные!</h2>
<div class="spoiler spoilerVisible"></div>
<br /><br />
"#;
        let html = Html::parse_document(markup);
        let h2 = html.select(&Selector::parse("h2").unwrap()).next().unwrap();
        println!("{}", h2.html());

        let next = super::next_sibling_element(&h2).unwrap();
        assert!(next
            .html()
            .starts_with("<div class=\"spoiler spoilerVisible\">"));
        println!("{}", next.html());
    }
}
