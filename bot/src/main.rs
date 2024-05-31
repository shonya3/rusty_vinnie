mod commands;
mod message_handler;
mod poe_newsletter;

use crate::poe_newsletter::spin_news_loop;
use dotenv::dotenv;
use fresh_news::{Subforum, WebsiteLanguage};
use message_handler::handle_message;
use poise::serenity_prelude::{self as serenity};

// Types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
pub type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("no DIVCORD_TOKEN env");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

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
        .await;
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!(
                "{}: Logged in as {}",
                chrono::Local::now().format("%a %T"),
                data_about_bot.user.name
            );

            tokio::join!(
                spin_news_loop(ctx.clone(), &WebsiteLanguage::En, &Subforum::News),
                spin_news_loop(ctx.clone(), &WebsiteLanguage::Ru, &Subforum::News),
                spin_news_loop(ctx.clone(), &WebsiteLanguage::En, &Subforum::PatchNotes),
                spin_news_loop(ctx.clone(), &WebsiteLanguage::Ru, &Subforum::PatchNotes),
            );
        }
        serenity::FullEvent::Message { new_message: msg } => handle_message(ctx, msg).await,
        _ => {}
    }
    Ok(())
}
