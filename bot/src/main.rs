use ::ea_live_updates::LiveUpdate;
use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity, futures::lock::Mutex};
use std::collections::HashSet;

mod channel;
mod commands;
pub mod ea_live_updates;
mod last_epoch;
mod message_handler;
mod poe_newsletter;
mod ready_handler;
mod status;
pub mod teasers;
mod unused;

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { .. } => ready_handler::handle_ready(ctx, data).await,
        serenity::FullEvent::Message { new_message: msg } => {
            message_handler::handle_message(ctx, msg).await
        }
        _ => {}
    }
    Ok(())
}

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
