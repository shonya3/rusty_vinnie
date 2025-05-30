use poe_teasers::{Lang, Teaser, TeasersForumThread};

#[test]
fn captures_multiple_content_links_from_one_teaser() {
    let forum_thread = TeasersForumThread::Poe2(Lang::En);

    let t = poe_teasers::parse_teasers_thread(
        &std::fs::read_to_string("./tests/newest_teaser_has_multiple_content_links.html").unwrap(),
        forum_thread,
    )
    .unwrap();

    assert_eq!(
        t.last().unwrap().to_owned(),
        Teaser {
            heading:
                "If you had to pick one monster from Oswald's journal to encounter in the Utzaal jungle, which would it be? Check out Oswald's notes on some more monsters from Path of Exile 2!"
                    .to_owned(),
                    images_urls: vec!["https://web.poecdn.com/public/news/2024-11-08/BlueSensibleRadars.png".to_owned(), "https://web.poecdn.com/public/news/2024-11-08/OrangePersonalFireplace.png".to_owned(), "https://web.poecdn.com/public/news/2024-11-08/PurplePlayfulPlatypus.png".to_owned(), "https://web.poecdn.com/public/news/2024-11-08/RedJoyfulHound.png".to_owned()],
                    videos_urls: vec![],
                    forum_thread
        }
    );
}

#[test]
fn parse_old_3_25_teasers_thread() {
    let markup = std::fs::read_to_string("./tests/3.25_some_teasers.html").unwrap();
    let forum_thread = TeasersForumThread::Poe1_3_25Russian;
    let vec = poe_teasers::parse_teasers_thread(&markup, forum_thread).unwrap();
    assert_eq!(vec, vec![
    Teaser {
        heading: "В дополнении Поселенцы Калгуура вы сможете начать схватки в Жатве всего одним действием.".to_owned(),
        images_urls: vec![],
        videos_urls: vec!["https://www.youtube.com/watch/7CwpLN5ryw4".to_owned()],
        forum_thread
    },
    Teaser {
        heading: "В Path of Exile: Поселенцы Калгуура вам больше не нужно нажимать на порталы в областях для их активации.".to_owned(),
        images_urls: vec![],
        videos_urls: vec!["https://www.youtube.com/watch/0Wd0mLXtteg".to_owned()],
        forum_thread
    },
     Teaser {
        heading: "Мы переработали качество предметов! Редкость предмета больше не имеет значения при использовании валюты для качества на неуникальные предметы. Вместо этого повышение качества теперь зависит от уровня предмета.".to_owned(),
        images_urls: vec![],
        videos_urls: vec!["https://www.youtube.com/watch/FlgP5NEQWbs".to_owned()],
        forum_thread
    },
    Teaser {
        heading: "Прибавки от качества на броне и оружии теперь мультипликативные!".to_owned(),
        images_urls: vec![],
        videos_urls: vec!["https://www.youtube.com/watch/T2bX9xXQOL8".to_owned()],
        forum_thread
    },
]);
}

#[test]
fn parse_poe2_teasers() {
    let markup = std::fs::read_to_string("./tests/poe2_some_teasers.html").unwrap();
    let forum_thread = TeasersForumThread::Poe2(Lang::Ru);
    let vec = poe_teasers::parse_teasers_thread(&markup, forum_thread).unwrap();
    assert_eq!(
        vec,
        vec![
            Teaser {
                heading: "У каждого уникального предмета в Path of Exile 2 есть собственные 2D-иконки и 3D-модели. Взгляните на некоторые знаковые уникальные предметы из Path of Exile, получившие новый внешний вид в Path of Exile 2.".to_owned(),
                images_urls: vec!["https://web.poecdn.com/public/news/2024-11-01/POE1Uniques.png".to_owned()],
                videos_urls: vec![],
                forum_thread
            },
            Teaser {
                heading: "С момента демонстрации класса Наёмник в Path of Exile 2, мы добавили гораздо больше огневой мощи в его арсенал. Оцените действие Гальванической гранаты на группу монстров и разрушительную силу Плазменного взрыва.".to_owned(),
                images_urls: vec![],
                videos_urls: vec!["https://vimeo.com/1025317638".to_owned()],
                forum_thread
            }
        ]
    );
}
