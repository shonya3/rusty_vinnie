use crate::SerenityContext;
use poise::serenity_prelude::{GuildId, UserId};
use std::{future::Future, time::Duration};

pub async fn watch_status<F, Fut>(get_status: impl Fn() -> Status, on_status_change: F)
where
    Fut: Future<Output = ()> + Send,
    F: Fn(Status) -> Fut,
{
    let mut status = get_status();
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        let new_status = get_status();
        if status != new_status {
            match new_status {
                Status::Online => {
                    status = Status::Online;
                    on_status_change(status).await;
                }
                Status::Offline => {
                    tokio::time::sleep(Duration::from_secs(600)).await;
                    if let Status::Offline = get_status() {
                        status = Status::Offline;
                        on_status_change(status).await;
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Online,
    Offline,
}

pub fn get_user_status(ctx: &SerenityContext, guild_id: &GuildId, user_id: &UserId) -> Status {
    match ctx.cache.guild(guild_id) {
        Some(guild) => match guild.presences.contains_key(user_id) {
            true => Status::Online,
            false => Status::Offline,
        },
        None => Status::Offline,
    }
}

pub fn get_kroiya_status(ctx: &SerenityContext) -> Status {
    get_user_status(
        ctx,
        &GuildId::new(356012941083934721),
        &UserId::new(182893458858442762),
    )
}
