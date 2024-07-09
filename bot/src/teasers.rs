use shuttle_persist::PersistInstance;
use teasers::{Content, Teaser};

use crate::{Context, Error};

pub fn _populate_teasers(persist: &PersistInstance) {
    let teas = vec![
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
    ];

    persist.save("teasers", teas).unwrap();
}

pub fn _get_teasers(
    persist: &PersistInstance,
) -> Result<Vec<Teaser>, shuttle_persist::PersistError> {
    persist.load("teasers")
}

/// Patchnotes links
#[poise::command(slash_command)]
pub async fn populate_teasers(ctx: Context<'_>) -> Result<(), Error> {
    _populate_teasers(&ctx.data().persist);
    ctx.say("Teasers populated").await?;

    Ok(())
}

/// Patchnotes links
#[poise::command(slash_command)]
pub async fn get_teasers(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let teas = _get_teasers(&data.persist).unwrap();
    ctx.say(serde_json::to_string(&teas).unwrap()).await?;

    Ok(())
}
