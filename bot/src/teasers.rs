use crate::{Context, Data, Error};
use poise::serenity_prelude::{ChannelId, Context as SerenityContext};
use shuttle_persist::PersistInstance;
use std::time::Duration;
use teasers::Teaser;

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

    for teaser in &thread_teasers {
        if !published_teasers.contains(teaser) {
            if let Err(err) = channel_id
                .say(&ctx, format!("{}\n{}", teaser.heading, &teaser.content))
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
                heading: "С момента демонстрации класса Наёмник в Path of Exile 2, мы добавили гораздо больше огневой мощи в его арсенал. Оцените действие Гальванической гранаты на группу монстров и разрушительную силу Плазменного взрыва.".to_owned(),
                content: "https://vimeo.com/1025317638".to_owned()
            },
            Teaser {
                heading: "У каждого уникального предмета в Path of Exile 2 есть собственные 2D-иконки и 3D-модели. Взгляните на некоторые знаковые уникальные предметы из Path of Exile, получившие новый внешний вид в Path of Exile 2.".to_owned(),
                content: "https://web.poecdn.com/public/news/2024-11-01/POE1Uniques.png".to_owned()
            }
        ];

    persist.save("teasers", teas).unwrap();
}
