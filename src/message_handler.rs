use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{EmojiId, Message, ReactionType};

pub async fn handle_message(ctx: &serenity::Context, msg: &Message) {
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
        if let Err(err) = msg.react(&ctx, emoji).await {
            eprintln!("Emoji reaction error: {err}");
        };
    }
}

enum VinnieEmoji {
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
