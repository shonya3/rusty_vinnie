use error::Error;
use scraper::{selectable::Selectable, ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
const USER_AGENT: &str = "rusty_vinnie/0.1 (contact: poeshonya3@gmail.com)";

pub mod error;

#[derive(Debug, Clone, Copy)]
pub enum Lang {
    Ru,
    En,
}

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

    let mut thread_teasers: Vec<Teaser> = teasers_post
        .select(&Selector::parse("h2").unwrap())
        .filter_map(|h2| {
            let spoiler_element = next_sibling_element(&h2)?;

            let iframes_links: Vec<String> = spoiler_element
                .select(&spoiler_content_iframe_selector)
                .filter_map(|iframe| {
                    iframe.attr("src").and_then(|src| match src {
                        src if src.starts_with("//www.youtube.com/") => {
                            Some(format!("https:{src}").replace("embed", "watch"))
                        }

                        src if src.starts_with("https://player.vimeo.com/video/") => {
                            Some(src.replace("player.vimeo.com/video/", "vimeo.com/"))
                        }

                        src if src.starts_with("https://") => Some(src.replace("embed", "watch")),

                        _ => None,
                    })
                })
                .collect();

            let imgs_links: Vec<String> = spoiler_element
                .select(&spoiler_content_img_selector)
                .filter_map(|img_el| img_el.attr("src").map(|attr| attr.to_string()))
                .collect();

            let mut contents: Vec<String> = Vec::new();
            contents.extend(iframes_links);
            contents.extend(imgs_links);

            Some(Teaser {
                heading: h2
                    .text()
                    .collect::<String>()
                    .trim()
                    .replace('\n', " ")
                    .replace('\t', ""),
                content: contents.join(" "),
            })
        })
        .collect();

    // Reverse teasers, because the newest one is on top of the teasers post.
    // So the order becomes [oldest, old, ..., newest]
    thread_teasers.reverse();

    Ok(thread_teasers)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
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

#[derive(Debug, Clone, Copy)]
pub enum Url {
    Poe2(Lang),
}

impl Url {
    pub fn as_str(&self) -> &'static str {
        match self {
            Url::Poe2(lang) => match lang {
                Lang::Ru => "https://ru.pathofexile.com/forum/view-thread/3584454",
                Lang::En => "https://www.pathofexile.com/forum/view-thread/3584453",
            },
        }
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
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
