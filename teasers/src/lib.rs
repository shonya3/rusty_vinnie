use error::Error;
use scraper::{ElementRef, Html, Selector};
use std::fmt::Display;

pub mod error;

pub async fn download_teasers_from_thread(url: &str) -> Result<Vec<Teaser>, Error> {
    let thread_markup = reqwest::get(url).await?.error_for_status()?.text().await?;
    std::fs::write("response.html", &thread_markup).unwrap();
    Ok(parse_teasers_thread(&thread_markup)?)
}

pub fn parse_teasers_thread(markup: &str) -> Result<Vec<Teaser>, ParseTeasersThreadError> {
    let html = Html::parse_document(markup);
    let teasers_post = html
        .select(&Selector::parse("table.forumTable tbody tr.newsPost").unwrap())
        .next()
        .ok_or(ParseTeasersThreadError::NoNewsPost)?;

    let mut vec: Vec<Teaser> = vec![];

    let content_iframe_selector = Selector::parse(".spoilerContent iframe").unwrap();
    for heading_element in teasers_post.select(&Selector::parse("h2").unwrap()) {
        let Some(content_container) = next_sibling_element(&heading_element) else {
            continue;
        };
        let Some(iframe) = content_container.select(&content_iframe_selector).next() else {
            continue;
        };
        let Some(youtube_attr_src) = iframe.attr("src") else {
            continue;
        };

        if youtube_attr_src.starts_with("//www.youtube.com/") {
            let url = format!("https:{youtube_attr_src}");
            let heading = heading_element
                .text()
                .collect::<String>()
                .replace(['\n', '\t'], "");

            vec.push(Teaser {
                heading,
                content: Content::YoutubeEmbedUrl(url),
            });
        }
    }

    Ok(vec)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Teaser {
    pub heading: String,
    pub content: Content,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    YoutubeEmbedUrl(String),
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
        let markup = std::fs::read_to_string("teasers.html").expect("Cannot find teasers.html");
        let teasers = super::parse_teasers_thread(&markup).unwrap();
        assert_eq!(teasers, vec![
    Teaser {
        heading: "Прибавки от качества на броне и оружии теперьмультипликативные!".to_owned(),
        content: Content::YoutubeEmbedUrl(
            "https://www.youtube.com/embed/T2bX9xXQOL8".to_owned(),
        ),
    },
    Teaser {
        heading: "Мы переработали качество предметов! Редкостьпредмета больше не имеет значения при использованиивалюты для качества на неуникальные предметы. Вместоэтого повышение качества теперь зависит от уровняпредмета.".to_owned(),
        content: Content::YoutubeEmbedUrl(
            "https://www.youtube.com/embed/FlgP5NEQWbs".to_owned(),
        ),
    },
    Teaser {
        heading: "В Path of Exile: Поселенцы Калгуура вам больше ненужно нажимать на порталы в областях для ихактивации.".to_owned(),
        content: Content::YoutubeEmbedUrl(
            "https://www.youtube.com/embed/0Wd0mLXtteg".to_owned(),
        ),
    },
    Teaser {
        heading: "В дополнении Поселенцы Калгуура вы сможете начатьсхватки в Жатве всего одним действием.".to_owned(),
        content: Content::YoutubeEmbedUrl(
            "https://www.youtube.com/embed/7CwpLN5ryw4".to_owned(),
        ),
    },
    Teaser {
        heading: "В Path of Exile: Поселенцы Калгуура мы добавляемнекоторые полезные улучшения. К примеру, эффектыудержания вроде Вестников и аур, теперь несбрасываются при смерти.".to_owned(),
        content: Content::YoutubeEmbedUrl(
            "https://www.youtube.com/embed/F4QpJGg9Bn0".to_owned(),
        ),
    },
]);
        println!("{teasers:#?}");
    }

    #[test]
    fn next_sibling() {
        let html = Html::parse_document(&std::fs::read_to_string("next_sibling.html").unwrap());
        let h2 = html.select(&Selector::parse("h2").unwrap()).next().unwrap();
        println!("{}", h2.html());

        let next = super::next_sibling_element(&h2).unwrap();
        assert!(next
            .html()
            .starts_with("<div class=\"spoiler spoilerVisible\">"));
        println!("{}", next.html());
    }
}
