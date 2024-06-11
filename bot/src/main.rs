mod commands;
mod message_handler;
mod poe_newsletter;
mod status;

use crate::poe_newsletter::spin_news_loop;
use chrono::{FixedOffset, Local};
use dotenv::dotenv;
use fresh_news::{Subforum, WebsiteLanguage};
use message_handler::handle_message;
use poise::serenity_prelude::{self as serenity, ChannelId};
use status::{get_kroiya_status, watch_status};

// Types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
pub type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    dotenv().ok();

    let token = secrets.get("DISCORD_TOKEN").expect("no DIVCORD_TOKEN env");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_PRESENCES;

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![commands::patchnotes(), commands::news()],
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
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            let working_channel = ChannelId::new(841929108829372460);

            working_channel
                .say(&ctx, format!("{}", Local::now().fixed_offset()))
                .await
                .unwrap();

            println!(
                "{}: Logged in as {}",
                chrono::Local::now().format("%a %T"),
                data_about_bot.user.name
            );

            let channel_id = ChannelId::new(356012941083934722);
            let say = |message: &'static str| async move {
                if let Err(err) = channel_id.say(ctx, message).await {
                    println!("Could not send message to channel: {err:#?}");
                };
            };

            let offset = FixedOffset::east_opt(3600);
            let offset = offset.as_ref();
            tokio::join!(
                watch_status(
                    || get_kroiya_status(ctx),
                    || say(":rabbit: пришел"),
                    || say(":rabbit: ушел"),
                ),
                spin_news_loop(ctx.clone(), &WebsiteLanguage::En, &Subforum::News, offset),
                spin_news_loop(ctx.clone(), &WebsiteLanguage::Ru, &Subforum::News, offset),
                spin_news_loop(
                    ctx.clone(),
                    &WebsiteLanguage::En,
                    &Subforum::PatchNotes,
                    offset
                ),
                spin_news_loop(
                    ctx.clone(),
                    &WebsiteLanguage::Ru,
                    &Subforum::PatchNotes,
                    offset
                ),
            );
        }
        serenity::FullEvent::Message { new_message: msg } => handle_message(ctx, msg).await,
        _ => {}
    }
    Ok(())
}
