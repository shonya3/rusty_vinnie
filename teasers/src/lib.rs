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
        .select(&Selector::parse("table.forumTable tbody tr.newsPost").unwrap())
        .next()
        .ok_or(ParseTeasersThreadError::NoNewsPost)?;

    Ok(teasers_post
        .select(&Selector::parse("h2").unwrap())
        .filter_map(|h2| {
            let youtube_attr_src = next_sibling_element(&h2).and_then(|content_container| {
                content_container
                    .select(&Selector::parse(".spoilerContent iframe").unwrap())
                    .next()
                    .and_then(|iframe| iframe.attr("src"))
            });
            let url = match youtube_attr_src {
                Some(attr) if attr.starts_with("//www.youtube.com/") => format!("https:{attr}"),
                Some(attr) if attr.starts_with("https") => attr.to_string(),
                _ => return None,
            };
            let url = url.replace("embed", "watch");
            let heading = h2.text().collect::<String>().replace(['\n', '\t'], "");

            Some(Teaser {
                heading,
                content: Content::YoutubeUrl(url),
            })
        })
        .collect())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Teaser {
    pub heading: String,
    pub content: Content,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Content {
    YoutubeUrl(String),
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

    use crate::{Content, Teaser};

    #[test]
    fn parse_teasers_thread() {
        let markup = std::fs::read_to_string("teasers.txt").expect("Cannot find teasers.txt");
        let teasers = super::parse_teasers_thread(&markup).unwrap();
        assert_eq!(teasers, vec![
    Teaser {
        heading: "Прибавки от качества на броне и оружии теперьмультипликативные!".to_owned(),
        content: Content::YoutubeUrl(
            "https://www.youtube.com/watch/T2bX9xXQOL8".to_owned(),
        ),
    },
    Teaser {
        heading: "Мы переработали качество предметов! Редкостьпредмета больше не имеет значения при использованиивалюты для качества на неуникальные предметы. Вместоэтого повышение качества теперь зависит от уровняпредмета.".to_owned(),
        content: Content::YoutubeUrl(
            "https://www.youtube.com/watch/FlgP5NEQWbs".to_owned(),
        ),
    },
    Teaser {
        heading: "В Path of Exile: Поселенцы Калгуура вам больше ненужно нажимать на порталы в областях для ихактивации.".to_owned(),
        content: Content::YoutubeUrl(
            "https://www.youtube.com/watch/0Wd0mLXtteg".to_owned(),
        ),
    },
    Teaser {
        heading: "В дополнении Поселенцы Калгуура вы сможете начатьсхватки в Жатве всего одним действием.".to_owned(),
        content: Content::YoutubeUrl(
            "https://www.youtube.com/watch/7CwpLN5ryw4".to_owned(),
        ),
    },
    Teaser {
        heading: "В Path of Exile: Поселенцы Калгуура мы добавляемнекоторые полезные улучшения. К примеру, эффектыудержания вроде Вестников и аур, теперь несбрасываются при смерти.".to_owned(),
        content: Content::YoutubeUrl(
            "https://www.youtube.com/watch/F4QpJGg9Bn0".to_owned(),
        ),
    },
]);
    }

    #[test]
    fn next_sibling() {
        let html = Html::parse_document(&std::fs::read_to_string("next_sibling.txt").unwrap());
        let h2 = html.select(&Selector::parse("h2").unwrap()).next().unwrap();
        println!("{}", h2.html());

        let next = super::next_sibling_element(&h2).unwrap();
        assert!(next
            .html()
            .starts_with("<div class=\"spoiler spoilerVisible\">"));
        println!("{}", next.html());
    }
}
