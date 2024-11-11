use teasers::Teaser;

#[test]
fn parse_old_3_25_teasers_thread() {
    let markup = std::fs::read_to_string("./tests/3.25_some_teasers.html").unwrap();
    let vec = teasers::parse_teasers_thread(&markup).unwrap();
    assert_eq!(vec, vec![
    Teaser {
        heading: "В дополнении Поселенцы Калгуура вы сможете начать схватки в Жатве всего одним действием.".to_owned(),
        content: "https://www.youtube.com/watch/7CwpLN5ryw4".to_owned(),
    },
    Teaser {
        heading: "В Path of Exile: Поселенцы Калгуура вам больше не нужно нажимать на порталы в областях для их активации.".to_owned(),
        content: "https://www.youtube.com/watch/0Wd0mLXtteg".to_owned(),
    },
     Teaser {
        heading: "Мы переработали качество предметов! Редкость предмета больше не имеет значения при использовании валюты для качества на неуникальные предметы. Вместо этого повышение качества теперь зависит от уровня предмета.".to_owned(),
        content: "https://www.youtube.com/watch/FlgP5NEQWbs".to_owned(),
    },
    Teaser {
        heading: "Прибавки от качества на броне и оружии теперь мультипликативные!".to_owned(),
        content: "https://www.youtube.com/watch/T2bX9xXQOL8".to_owned(),
    },
]);
}

#[test]
fn parse_poe2_teasers() {
    let markup = std::fs::read_to_string("./tests/poe2_some_teasers.html").unwrap();
    let vec = teasers::parse_teasers_thread(&markup).unwrap();
    assert_eq!(
        vec,
        vec![
            Teaser {
                heading: "У каждого уникального предмета в Path of Exile 2 есть собственные 2D-иконки и 3D-модели. Взгляните на некоторые знаковые уникальные предметы из Path of Exile, получившие новый внешний вид в Path of Exile 2.".to_owned(),
                content: "https://web.poecdn.com/public/news/2024-11-01/POE1Uniques.png".to_owned()
            },
            Teaser {
                heading: "С момента демонстрации класса Наёмник в Path of Exile 2, мы добавили гораздо больше огневой мощи в его арсенал. Оцените действие Гальванической гранаты на группу монстров и разрушительную силу Плазменного взрыва.".to_owned(),
                content: "https://vimeo.com/1025317638".to_owned()
            }
        ]
    );
}
