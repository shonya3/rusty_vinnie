mod commands;
pub mod ea_live_updates;
mod last_epoch;
mod message_handler;
mod poe_newsletter;
mod status;
pub mod teasers;
mod unused;

use std::collections::HashSet;

use crate::poe_newsletter::spin_news_loop;
use ::ea_live_updates::LiveUpdate;
use chrono::FixedOffset;
use dotenv::dotenv;
use fresh_news::{Subforum, WebsiteLanguage};
use last_epoch::{watch_lastepoch, Subforum as LastEpochSubforum};
use message_handler::handle_message;
use poise::serenity_prelude::{self as serenity, futures::lock::Mutex, ChannelId};
use status::{get_kroiya_status, watch_status};
use teasers::spin_teasers_loop;

// Types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
pub type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    #[allow(unused)]
    published_live_updates: Mutex<HashSet<LiveUpdate>>,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    println!("App Start");
    dotenv().ok();

    let token = secrets.get("DISCORD_TOKEN").expect("no DISCORD_TOKEN env");
    println!("{token}");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_PRESENCES;

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    published_live_updates: Mutex::new(HashSet::new()),
                })
            })
        })
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![
                commands::patchnotes(),
                commands::news(),
                // crate::teasers::populate_teasers(),
                crate::commands::ascendancies1(),
                crate::commands::ascendancies2(),
            ],
            ..Default::default()
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { .. } => {
            println!("Bot is ready");
            let _working_channel = ChannelId::new(841929108829372460);
            let main_channel = ChannelId::new(356012941083934722);
            let _archer_mains_channel = ChannelId::new(356013349496029184);
            let say = |message: &'static str| async move {
                if let Err(err) = main_channel.say(ctx, message).await {
                    println!("Could not send message to channel: {err:#?}");
                };
            };

            let offset = FixedOffset::east_opt(3600);
            // winter london time
            // let offset = FixedOffset::east_opt(0);
            let offset = offset.as_ref();

            tokio::join!(
                watch_status(
                    || get_kroiya_status(ctx),
                    || say(":rabbit: пришел"),
                    || say(":rabbit: ушел"),
                ),
                watch_lastepoch(ctx, LastEpochSubforum::Announcements),
                watch_lastepoch(ctx, LastEpochSubforum::DeveloperBlogs),
                watch_lastepoch(ctx, LastEpochSubforum::News),
                watch_lastepoch(ctx, LastEpochSubforum::PatchNotes),
                spin_teasers_loop(ctx, data, &[], &_archer_mains_channel),
                spin_news_loop(ctx, &WebsiteLanguage::En, &Subforum::News, offset),
                spin_news_loop(ctx, &WebsiteLanguage::Ru, &Subforum::News, offset),
                spin_news_loop(ctx, &WebsiteLanguage::En, &Subforum::PatchNotes, offset),
                spin_news_loop(ctx, &WebsiteLanguage::Ru, &Subforum::PatchNotes, offset),
                spin_news_loop(
                    ctx,
                    &WebsiteLanguage::En,
                    &Subforum::EarlyAccessPatchNotesEn,
                    offset
                ),
                spin_news_loop(
                    ctx,
                    &WebsiteLanguage::Ru,
                    &Subforum::EarlyAccessPatchNotesRu,
                    offset
                ),
                spin_news_loop(
                    ctx,
                    &WebsiteLanguage::En,
                    &Subforum::EarlyAccessAnnouncementsEn,
                    offset
                ),
                spin_news_loop(
                    ctx,
                    &WebsiteLanguage::Ru,
                    &Subforum::EarlyAccessAnnouncementsRu,
                    offset
                ),
            );
        }
        serenity::FullEvent::Message { new_message: msg } => handle_message(ctx, msg).await,
        _ => {}
    }
    Ok(())
}
