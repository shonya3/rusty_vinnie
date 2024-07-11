use crate::{Context, Data, Error};
use poise::serenity_prelude::{ChannelId, Context as SerenityContext};
use shuttle_persist::PersistInstance;
use std::time::Duration;
use teasers::{Content, Teaser};

pub async fn spin_teasers_loop(
    ctx: &SerenityContext,
    data: &Data,
    url: &str,
    channel_id: &ChannelId,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(360));
    loop {
        publish_new_teasers(ctx, data, url, channel_id).await;
        interval.tick().await;
    }
}

pub async fn publish_new_teasers(
    ctx: &SerenityContext,
    data: &Data,
    url: &str,
    channel_id: &ChannelId,
) {
    let persist = &data.persist;
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
            let Content::YoutubeUrl(content) = &thread_teaser.content;

            if let Err(err) = channel_id
                .say(&ctx, format!("{}\n{}", thread_teaser.heading, content))
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
    let teas = load_published_teasers(&data.persist)
        .into_iter()
        .map(|t| t.heading)
        .collect::<Vec<String>>()
        .join("\n");
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
    ];

    persist.save("teasers", teas).unwrap();
}
