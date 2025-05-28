use error::Error;
use scraper::{selectable::Selectable, ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod error;

pub async fn download_teasers_from_thread(
    forum_thread: TeasersForumThread,
) -> Result<Vec<Teaser>, Error> {
    let thread_markup = http::text(forum_thread.url()).await?;
    Ok(parse_teasers_thread(&thread_markup, forum_thread)?)
}

pub fn parse_teasers_thread(
    markup: &str,
    forum_thread: TeasersForumThread,
) -> Result<Vec<Teaser>, ParseTeasersThreadError> {
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

            let videos_urls: Vec<String> = spoiler_element
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

            let images_urls: Vec<String> = spoiler_element
                .select(&spoiler_content_img_selector)
                .filter_map(|img_el| img_el.attr("src").map(|attr| attr.to_string()))
                .collect();

            Some(Teaser {
                heading: h2
                    .text()
                    .collect::<String>()
                    .trim()
                    .replace('\n', " ")
                    .replace('\t', ""),
                images_urls,
                videos_urls,
                forum_thread,
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
    pub images_urls: Vec<String>,
    pub videos_urls: Vec<String>,
    pub forum_thread: TeasersForumThread,
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Hash, Eq)]
pub enum TeasersForumThread {
    Poe2(Lang),
    Poe1_3_25Russian,
    Poe2_02(Lang),
    Poe1_3_26Ru,
    Poe1_3_26En,
}

impl TeasersForumThread {
    pub fn url(&self) -> &'static str {
        match self {
            &TeasersForumThread::Poe2(lang) => match lang {
                Lang::Ru => "https://ru.pathofexile.com/forum/view-thread/3584454",
                Lang::En => "https://www.pathofexile.com/forum/view-thread/3584453",
            },
            TeasersForumThread::Poe1_3_25Russian => {
                "https://ru.pathofexile.com/forum/view-thread/3530604/page/1"
            }
            TeasersForumThread::Poe2_02(lang) => match lang {
                Lang::Ru => "https://ru.pathofexile.com/forum/view-thread/3726161",
                Lang::En => "https://www.pathofexile.com/forum/view-thread/3726160",
            },
            TeasersForumThread::Poe1_3_26Ru => {
                "https://ru.pathofexile.com/forum/view-thread/3784649"
            }
            TeasersForumThread::Poe1_3_26En => {
                "https://www.pathofexile.com/forum/view-thread/3784648"
            }
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            TeasersForumThread::Poe2(lang) => match lang {
                Lang::Ru => "Тизеры Path of Exile 2",
                Lang::En => "Path of Exile 2 Teasers",
            },
            TeasersForumThread::Poe1_3_25Russian => "Тизеры Path of Exile: Поселенцы Калгуура",
            TeasersForumThread::Poe2_02(lang) => match lang {
                Lang::Ru => "Тизеры Path of Exile 2 0.2.0",
                Lang::En => "Path of Exile 2 - 0.2.0 Teasers",
            },
            TeasersForumThread::Poe1_3_26Ru => "Тизеры Path of Exile: Секреты Атласа",
            TeasersForumThread::Poe1_3_26En => "Path of Exile: Secrets of the Atlas Teasers",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Lang {
    Ru,
    En,
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
