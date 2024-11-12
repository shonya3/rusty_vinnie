use crate::{Context, Data, Error};
use poise::{
    serenity_prelude::{ChannelId, Context as SerenityContext, CreateEmbed, CreateEmbedAuthor},
    CreateReply,
};
use shuttle_persist::PersistInstance;
use std::{collections::HashSet, time::Duration};
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

    let mut set = HashSet::<Teaser>::from_iter(published_teasers);
    set.extend(thread_teasers);

    let unique_teasers: Vec<Teaser> = set.into_iter().collect();

    if let Err(err) = persist.save("teasers", unique_teasers) {
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

fn create_vinnie_bot_author_embed() -> CreateEmbedAuthor {
    CreateEmbedAuthor::new("Vinnie The Bot")
        .icon_url("https://discord.com/assets/ca24969f2fd7a9fb03d5.png")
        .url("https://github.com/shonya3/rusty_vinnie")
}

#[poise::command(slash_command)]
#[allow(clippy::field_reassign_with_default)]
pub async fn get_latest_teaser(ctx: Context<'_>) -> Result<(), Error> {
    let url = "https://www.pathofexile.com/forum/view-thread/3584453";
    let embed = CreateEmbed::new()
        .title("Poe Teaser")
        .url(url)
        .author(create_vinnie_bot_author_embed())
        .description("If you had to pick one monster from Oswald's journal to encounter in the Utzaal jungle, which would it be? Check out Oswald's notes on some more monsters from Path of Exile 2!");

    let links: Vec<&str> = vec![
        "https://web.poecdn.com/public/news/2024-11-08/BlueSensibleRadars.png",
        "https://web.poecdn.com/public/news/2024-11-08/OrangePersonalFireplace.png",
        "https://web.poecdn.com/public/news/2024-11-08/PurplePlayfulPlatypus.png",
        "https://web.poecdn.com/public/news/2024-11-08/RedJoyfulHound.png",
    ];
    let images_embeds: Vec<_> = links
        .into_iter()
        .map(|image_url| CreateEmbed::new().image(image_url).url(url))
        .collect();

    let mut reply = CreateReply::default();
    reply.embeds = vec![embed];
    reply.embeds.extend(images_embeds);

    ctx.send(reply).await?;

    Ok(())
}

// #[poise::command(slash_command)]
// #[allow(clippy::field_reassign_with_default)]
// pub async fn get_latest_teaser(ctx: Context<'_>) -> Result<(), Error> {
//     let data = ctx.data();
//     let mut teas = load_published_teasers(&data.persist);

//     teas.sort_by(|a, b| b.content.cmp(&a.content));
//     let teaser = teas.first().to_owned();

//     match teaser {
//         Some(teaser) => {
//             let url = "https://www.youtube.com/watch?v=CagIhaIoqtg";
//             let embed = CreateEmbed::new()
//                 .title("Poe Teaser")
//                 .url(url)
//                 .author(create_vinnie_bot_author_embed())
//                 .description(&teaser.heading);

//             let links = teaser.content.split(" ").collect::<Vec<_>>();
//             let images_embeds: Vec<_> = links
//                 .into_iter()
//                 .map(|image_url| CreateEmbed::new().image(image_url).url(url))
//                 .collect();

//             // let img_embeds = EmbedImage::

//             let mut reply = CreateReply::default();
//             reply.embeds = vec![embed];
//             reply.embeds.extend(images_embeds);

//             // EmbedImage

//             ctx.send(reply).await?;
//         }
//         None => {
//             let embed = CreateEmbed::new()
//                 .title("PoE Teaser")
//                 .description("description here");

//             let reply = CreateReply::default().embed(embed);

//             ctx.send(reply).await?;
//         }
//     };

//     Ok(())
// }

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
