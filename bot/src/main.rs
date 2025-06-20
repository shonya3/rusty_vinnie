use dotenv::dotenv;
use ::ea_live_updates::LiveUpdate;
use futures::lock::Mutex;
use poise::serenity_prelude::{self as serenity};
use std::{collections::HashSet, sync::Arc};

mod channel;
mod commands;
mod interval;
mod last_epoch;
mod message;
mod message_handler;
mod poe_newsletter;
pub mod poe_teasers;
mod ready_handler;
mod status;
#[allow(unused)]
mod ea_live_updates;
mod unused;

pub const EMBED_DESCRIPTION_MAX_CHARS: usize = 4096;
pub const EMBED_DESCRIPTION_CUSTOM_MAX_CHARS: usize = 1000;

pub type DbClient = libsql::Database;

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
    pub db: Arc<DbClient>,
    pub published_live_updates: Arc<Mutex<HashSet<LiveUpdate>>>
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
                
                let db_url = secrets.get("DB_URL")
                    .expect("DB_URL environment variable not set.");
                let db_token = secrets.get("DB_TOKEN")
                    .expect("DB_TOKEN environment variable not set."); 

                let db = libsql::Builder::new_remote(db_url, db_token)
                    .build()
                    .await
                    .expect("Failed to create Database from environment. Ensure DB_URL and DB_TOKEN are set and valid.");
            
                let conn = db.connect().expect("Failed to get connection for schema setup");
                poe_teasers::db_layer::ensure_schema_exists(&conn).await
                    .expect("Failed to ensure database schema exists.");
                Ok(Data {db:Arc::new(db), published_live_updates: Default::default() })
            })
        })
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![last_epoch::epoch_thread()],
            ..Default::default()
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
