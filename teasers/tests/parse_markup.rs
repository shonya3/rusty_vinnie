use teasers::{Content, Teaser};

#[test]
fn parse_old_3_25_teasers_thread() {
    let markup = std::fs::read_to_string("./tests/3.25_teasers.html").unwrap();
    let vec = teasers::parse_teasers_thread(&markup).unwrap();
    assert_eq!(vec, vec![
    Teaser {
        heading: "Прибавки от качества на броне и оружии теперь мультипликативные!".to_owned(),
        content: Content::YoutubeUrl(
            "https://www.youtube.com/watch/T2bX9xXQOL8".to_owned(),
        ),
    },
    Teaser {
        heading: "Мы переработали качество предметов! Редкость предмета больше не имеет значения при использовании валюты для качества на неуникальные предметы. Вместо этого повышение качества теперь зависит от уровня предмета.".to_owned(),
        content: Content::YoutubeUrl(
            "https://www.youtube.com/watch/FlgP5NEQWbs".to_owned(),
        ),
    },
    Teaser {
        heading: "В Path of Exile: Поселенцы Калгуура вам больше не нужно нажимать на порталы в областях для их активации.".to_owned(),
        content: Content::YoutubeUrl(
            "https://www.youtube.com/watch/0Wd0mLXtteg".to_owned(),
        ),
    },
    Teaser {
        heading: "В дополнении Поселенцы Калгуура вы сможете начать схватки в Жатве всего одним действием.".to_owned(),
        content: Content::YoutubeUrl(
            "https://www.youtube.com/watch/7CwpLN5ryw4".to_owned(),
        ),
    },

]);
}
