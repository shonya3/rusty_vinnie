use poise::serenity_prelude::{EmojiId, ReactionType};
use std::fmt::Display;

#[derive(Copy, Clone)]
pub enum Emoji {
    // Custom
    Jaba,
    Nivazmojna,
    Zdruste,
    RustHappy,

    // Unicode
    Utrechka,
    Crab,
    Eggplant,
}

impl From<Emoji> for ReactionType {
    fn from(emoji: Emoji) -> Self {
        emoji.reaction()
    }
}

impl Emoji {
    pub fn reaction(&self) -> ReactionType {
        match self {
            Emoji::Jaba => Custom::Jaba.reaction(),
            Emoji::Nivazmojna => Custom::Nivazmojna.reaction(),
            Emoji::Zdruste => Custom::Zdruste.reaction(),
            Emoji::RustHappy => Custom::RustHappy.reaction(),

            Emoji::Utrechka => Unicode::Utrechka.reaction(),
            Emoji::Crab => Unicode::Crab.reaction(),
            Emoji::Eggplant => Unicode::Eggplant.reaction(),
        }
    }

    pub fn all() -> [Emoji; 7] {
        [
            // Custom
            Emoji::Jaba,
            Emoji::Nivazmojna,
            Emoji::Zdruste,
            Emoji::RustHappy,
            // Unicode
            Emoji::Utrechka,
            Emoji::Crab,
            Emoji::Eggplant,
        ]
    }
}

impl Display for Emoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Emoji::Jaba => Custom::Jaba.fmt(f),
            Emoji::Nivazmojna => Custom::Nivazmojna.fmt(f),
            Emoji::Zdruste => Custom::Zdruste.fmt(f),
            Emoji::RustHappy => Custom::RustHappy.fmt(f),

            Emoji::Utrechka => Unicode::Utrechka.fmt(f),
            Emoji::Crab => Unicode::Crab.fmt(f),
            Emoji::Eggplant => Unicode::Eggplant.fmt(f),
        }
    }
}

impl From<Custom> for Emoji {
    fn from(custom: Custom) -> Self {
        match custom {
            Custom::Jaba => Emoji::Jaba,
            Custom::Nivazmojna => Emoji::Nivazmojna,
            Custom::Zdruste => Emoji::Zdruste,
            Custom::RustHappy => Emoji::RustHappy,
        }
    }
}

impl From<Unicode> for Emoji {
    fn from(unicode: Unicode) -> Self {
        match unicode {
            Unicode::Utrechka => Emoji::Utrechka,
            Unicode::Crab => Emoji::Crab,
            Unicode::Eggplant => Emoji::Eggplant,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Unicode {
    Utrechka,
    Crab,
    Eggplant,
}

impl Unicode {
    fn char(&self) -> char {
        match self {
            Unicode::Utrechka => '🤓',
            Unicode::Crab => '🦀',
            Unicode::Eggplant => '🍆',
        }
    }

    fn reaction(&self) -> ReactionType {
        ReactionType::Unicode(self.to_string())
    }
}

impl Display for Unicode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}

#[derive(Debug, Clone, Copy)]
enum Custom {
    Jaba,
    Nivazmojna,
    Zdruste,
    RustHappy,
}

impl Custom {
    fn name(&self) -> &'static str {
        match self {
            Custom::Jaba => "jaba",
            Custom::Nivazmojna => "nivazmojna",
            Custom::Zdruste => "zdruste",
            Custom::RustHappy => "rusthappy",
        }
    }

    fn id(&self) -> u64 {
        match self {
            Custom::Jaba => 637684829114204175,
            Custom::Nivazmojna => 850123166923882558,
            Custom::Zdruste => 1082770484036374639,
            Custom::RustHappy => 1082770391845585016,
        }
    }

    fn reaction(&self) -> ReactionType {
        ReactionType::Custom {
            animated: false,
            id: EmojiId::new(self.id()),
            name: Some(self.name().to_string()),
        }
    }
}

impl Display for Custom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<:{}:{}>", self.name(), self.id())
    }
}
