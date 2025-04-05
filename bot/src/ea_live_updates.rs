use std::{collections::HashMap, time::Duration};

use ea_live_updates::{LiveUpdate, LiveUpdatesThread};
use poise::serenity_prelude::{
    ChannelId, Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateMessage,
};

use crate::Data;

pub async fn spin_ea_live_updates_loop(
    ctx: &SerenityContext,
    data: &Data,
    forum_threads: &[LiveUpdatesThread],
    channel_id: &ChannelId,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(360));
    loop {
        for forum_thread in forum_threads {
            publish_new_ea_live_updates(ctx, data, *forum_thread, channel_id).await;
        }
        interval.tick().await;
    }
}

// pub async fn publish_new_ea_live_updates(
//     ctx: &SerenityContext,
//     _data: &Data,
//     live_updates_thread: LiveUpdatesThread,
//     channel_id: &ChannelId,
// ) {
//     let ea_updates = match ea_live_updates::get_live_updates(live_updates_thread).await {
//         Ok(updates) => updates,
//         Err(err) => {
//             println!("Could not get ea live updates. {live_updates_thread:#?} {err}");
//             return;
//         }
//     };
//     let already_seen_updates = load_published_updates();

//     let not_seen_updates = ea_updates
//         .iter()
//         .filter(|update| !already_seen_updates.contains(update))
//         .collect::<Vec<_>>();

//     send_live_updates(ctx, channel_id, &not_seen_updates)
//         .await
//         .unwrap_or_else(|err| eprintln!("publish_new_updates Error: {err}"));

//     let mut set = HashSet::<LiveUpdate>::from_iter(already_seen_updates);
//     set.extend(ea_updates);

//     let _unique_updates: Vec<LiveUpdate> = set.into_iter().collect();

//     if let Err(err) = save_published_updates() {
//         println!("Could not persist ea live teasers: {err}");
//     }
// }

pub async fn publish_new_ea_live_updates(
    ctx: &SerenityContext,
    data: &Data,
    live_updates_thread: LiveUpdatesThread,
    channel_id: &ChannelId,
) {
    let ea_updates = match ea_live_updates::get_live_updates(live_updates_thread).await {
        Ok(updates) => updates,
        Err(err) => {
            println!("Could not get ea live updates. {live_updates_thread:#?} {err}");
            return;
        }
    };

    let mut published = data.published_live_updates.lock().await;

    let not_seen_updates = ea_updates
        .iter()
        .filter(|update| !published.contains(update))
        .collect::<Vec<_>>();

    send_live_updates(ctx, channel_id, &not_seen_updates)
        .await
        .unwrap_or_else(|err| eprintln!("publish_new_updates Error: {err}"));

    for u in not_seen_updates {
        published.insert(u.clone());
    }
}

async fn send_live_updates(
    ctx: &SerenityContext,
    channel_id: &ChannelId,
    updates: &[&LiveUpdate],
) -> Result<(), String> {
    let mut map: HashMap<u32, Vec<&LiveUpdate>> = HashMap::new();
    updates.iter().for_each(|update| {
        map.entry(update.day).or_default().push(update);
    });

    let embeds = map
        .iter()
        .filter_map(|(_, updates)| {
            let thread = updates.first().map(|u| u.thread)?;
            Some(
                CreateEmbed::new()
                    .title(thread.title())
                    .url(thread.url())
                    .author(create_vinnie_bot_author_embed())
                    .fields(
                        updates
                            .iter()
                            .map(|update| (update.heading.clone(), update.content.clone(), false)),
                    ),
            )
        })
        .collect::<Vec<_>>();
    if embeds.is_empty() {
        return Ok(());
    }

    let message = CreateMessage::new().embeds(embeds);
    if let Err(err) = channel_id.send_message(&ctx, message).await {
        return Err(format!("Could not send live update to {channel_id}. {err}"));
    }

    Ok(())
}

// // TODO Use the actual storage
// fn load_published_updates() -> Vec<LiveUpdate> {
//     Vec::new()
// }

// // TODO Use the actual storage
// fn save_published_updates() -> Result<(), String> {
//     Ok(())
// }

fn create_vinnie_bot_author_embed() -> CreateEmbedAuthor {
    CreateEmbedAuthor::new("Rusty Vinnie")
        .icon_url("https://cdn.discordapp.com/app-icons/1139087605003202610/00040381fd8cae4be71e1b9b57723806.png")
        .url("https://github.com/shonya3/rusty_vinnie")
}
