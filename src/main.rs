use std::env::var;

use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity, CacheHttp, EmojiId, ReactionType};

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = var("DISCORD_TOKEN").expect("no DIVCORD_TOKEN env");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }))
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
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
            println!("Logged in as {}", data_about_bot.user.name);
            // for guild in ctx.cache.guilds() {
            //     for emoji in guild.emojis(&ctx).await.unwrap() {
            //         if emoji.name == "zdruste" {
            //             dbg!(&emoji);
            //         }
            //     }
            // }
        }
        serenity::FullEvent::Message { new_message: msg } => {
            let mut emojis: Vec<VinnieEmoji> = vec![];

            let m = msg.content.as_str();

            if m.contains("jab") || m.contains("жаб") {
                emojis.push(VinnieEmoji::Jaba);
            };

            if m.contains("нивазможн") || m.contains("невозможн") || m.contains("nivazmojn")
            {
                emojis.push(VinnieEmoji::Nivazmojna);
            };

            if m.contains("утр") {
                emojis.push(VinnieEmoji::Utrechka);
            };

            if m.contains("rust") || m.contains("раст") {
                emojis.push(VinnieEmoji::Zdruste);
            };

            for emoji in emojis {
                if let Err(err) = msg.react(ctx, emoji).await {
                    eprintln!("Emoji reaction error: {err}");
                };
            }
        }
        _ => {}
    }
    Ok(())
}

pub enum VinnieEmoji {
    Jaba,
    Nivazmojna,
    Utrechka,
    Zdruste,
}

impl VinnieEmoji {
    pub fn id(&self) -> ReactionType {
        match self {
            VinnieEmoji::Jaba => ReactionType::Custom {
                animated: false,
                id: EmojiId::new(637684829114204175),
                name: Some(String::from("jaba")),
            },
            VinnieEmoji::Nivazmojna => ReactionType::Custom {
                animated: false,
                id: EmojiId::new(850123166923882558),
                name: Some(String::from("nivazmojna")),
            },
            VinnieEmoji::Utrechka => ReactionType::Unicode(String::from("🤓")),
            VinnieEmoji::Zdruste => ReactionType::Custom {
                animated: false,
                id: EmojiId::new(1082770484036374639),
                name: Some(String::from("zdruste")),
            },
        }
    }
}

impl From<VinnieEmoji> for ReactionType {
    fn from(value: VinnieEmoji) -> Self {
        value.id()
    }
}
