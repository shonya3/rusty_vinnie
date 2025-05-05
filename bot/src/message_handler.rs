use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{EmojiId, Message, ReactionType};
use rand::seq::SliceRandom;

pub async fn handle_message(ctx: &serenity::Context, msg: &Message) {
    let mut emojis: Vec<VinnieEmoji> = vec![];

    let message = msg.content.to_lowercase();
    let has = |s: &str| message.contains(s);
    let any = |patterns: &[&str]| patterns.iter().any(|p| has(p));

    if any(&["жаб", "jab"]) {
        emojis.push(VinnieEmoji::Jaba);
    };

    if any(&["нивазможн", "невозможн", "nivazmojn"]) {
        emojis.push(VinnieEmoji::Nivazmojna);
    };

    if any(&["утр", "бдо"]) {
        emojis.push(VinnieEmoji::Utrechka);
    };

    if any(&["икр", "баклажан"]) {
        emojis.push(VinnieEmoji::Eggplant);
    }

    if any(&["rust", "раст", "краб", "crab", "🦀"]) {
        let emoji = [
            VinnieEmoji::Zdruste,
            VinnieEmoji::Crab,
            VinnieEmoji::RustHappy,
        ]
        .choose(&mut rand::thread_rng())
        .unwrap_or(&VinnieEmoji::Zdruste);

        emojis.push(*emoji);
    }

    for emoji in emojis {
        if let Err(err) = msg.react(&ctx, emoji).await {
            eprintln!("Emoji reaction error: {err}");
        };
    }
}

#[derive(Copy, Clone)]
enum VinnieEmoji {
    Jaba,
    Nivazmojna,
    Utrechka,
    Zdruste,
    Crab,
    RustHappy,
    Eggplant,
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
            VinnieEmoji::Crab => ReactionType::Unicode(String::from("🦀")),
            VinnieEmoji::RustHappy => ReactionType::Custom {
                animated: false,
                id: EmojiId::new(1082770391845585016),
                name: Some(String::from("rusthappy")),
            },
            VinnieEmoji::Eggplant => ReactionType::Unicode(String::from("🍆")),
        }
    }
}

impl From<VinnieEmoji> for ReactionType {
    fn from(value: VinnieEmoji) -> Self {
        value.id()
    }
}
