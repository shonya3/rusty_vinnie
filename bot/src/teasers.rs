use crate::{Context, Error};
use poise::serenity_prelude::ChannelId;
use shuttle_persist::PersistInstance;
use teasers::{Content, Teaser};

pub async fn run_teasers_task(ctx: Context<'_>, url: &str, channel_id: &ChannelId) {
    let persist = &ctx.data().persist;
    let thread_teasers = match teasers::download_teasers_from_thread(url).await {
        Ok(teas) => teas,
        Err(err) => {
            println!("Could not download thread teasers. {err}");
            return;
        }
    };
    let published_teasers = load_published_teasers(persist);

    for thread_teaser in &thread_teasers {
        if !published_teasers.contains(thread_teaser) {
            let Content::YoutubeEmbedUrl(content) = &thread_teaser.content;

            if let Err(err) = channel_id
                .say(ctx, format!("{}\n{}", thread_teaser.heading, content))
                .await
            {
                println!("Could not send teaser to chat: {err}");
            };
        };
    }

    if let Err(err) = persist.save("teasers", thread_teasers) {
        println!("Could not persist thread teasers: {err}");
    };
}

/// Patchnotes links
#[poise::command(slash_command)]
pub async fn populate_teasers(ctx: Context<'_>) -> Result<(), Error> {
    _populate_teasers(&ctx.data().persist);
    ctx.say("Teasers populated").await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn get_teasers(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let teas = load_published_teasers(&data.persist);
    ctx.say(serde_json::to_string(&teas).unwrap()).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn clear_teasers(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    data.persist.remove("teasers")?;
    ctx.say("Teasers cleared").await?;

    Ok(())
}

fn load_published_teasers(persist: &PersistInstance) -> Vec<Teaser> {
    match persist.load::<Vec<Teaser>>("teasers") {
        Ok(teasers) => teasers,
        Err(err) => {
            {
                println!("Could not load persisted teasers: {err}");
            }
            vec![]
        }
    }
}

fn _populate_teasers(persist: &PersistInstance) {
    let teas = vec![
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
