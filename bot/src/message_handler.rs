use crate::{emoji::Emoji, SerenityContext};
use poise::serenity_prelude::Message;
use rand::seq::IndexedRandom;

pub async fn handle_message(ctx: &SerenityContext, msg: &Message) {
    let mut emojis: Vec<Emoji> = vec![];

    let message = msg.content.to_lowercase();
    let has = |s: &str| message.contains(s);
    let any = |patterns: &[&str]| patterns.iter().any(|p| has(p));

    if any(&["жаб", "jab"]) {
        emojis.push(Emoji::Jaba);
    };

    if any(&["нивазможн", "невозможн", "nivazmojn"]) {
        emojis.push(Emoji::Nivazmojna);
    };

    if any(&["утр", "бдо"]) {
        emojis.push(Emoji::Utrechka);
    };

    if any(&["икр", "баклажан"]) {
        emojis.push(Emoji::Eggplant);
    }

    if any(&["rust", "раст", "краб", "crab", "🦀"]) {
        let emoji = [Emoji::Zdruste, Emoji::Crab, Emoji::RustHappy]
            .choose(&mut rand::rng())
            .unwrap_or(&Emoji::Zdruste);

        emojis.push(*emoji);
    }

    for emoji in emojis {
        if let Err(err) = msg.react(&ctx, emoji).await {
            eprintln!("Emoji reaction error: {err}");
        };
    }
}
