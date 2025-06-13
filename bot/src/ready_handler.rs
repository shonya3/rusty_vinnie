use crate::{
    channel::AppChannel,
    last_epoch::{self, Subforum as LastEpochSubforum},
    poe_newsletter,
    status::{get_kroiya_status, watch_status},
    Data,
};
use chrono::FixedOffset;
use poe_forum::{Subforum, WebsiteLanguage};
use poise::serenity_prelude::{self as serenity};

pub async fn handle_ready(ctx: &serenity::Context, data: &Data) {
    println!("Bot is ready");

    set_watchers(ctx, data).await;
}

async fn set_watchers(ctx: &serenity::Context, _data: &Data) {
    tokio::join!(
        watch_status(
            || get_kroiya_status(ctx),
            || AppChannel::General.say(ctx, ":rabbit: пришел"),
            || AppChannel::General.say(ctx, ":rabbit: ушел"),
        ),
        last_epoch::watch_subforums(
            ctx,
            vec![
                LastEpochSubforum::Announcements,
                LastEpochSubforum::DeveloperBlogs,
                LastEpochSubforum::News,
                LastEpochSubforum::PatchNotes,
            ],
        ),
        poe_newsletter::watch_subforums(
            ctx,
            vec![
                (WebsiteLanguage::En, Subforum::News),
                (WebsiteLanguage::Ru, Subforum::News),
                (WebsiteLanguage::En, Subforum::PatchNotes),
                (WebsiteLanguage::Ru, Subforum::PatchNotes),
                (WebsiteLanguage::En, Subforum::EarlyAccessPatchNotesEn),
                (WebsiteLanguage::Ru, Subforum::EarlyAccessPatchNotesRu),
                (WebsiteLanguage::En, Subforum::EarlyAccessAnnouncementsEn),
                (WebsiteLanguage::Ru, Subforum::EarlyAccessAnnouncementsRu),
            ],
            Timezone::BritishSummer.offset(),
        ),
    );
}

#[allow(dead_code)]
pub enum Timezone {
    BritishWinter,
    BritishSummer,
}

impl Timezone {
    pub fn offset(&self) -> Option<FixedOffset> {
        match self {
            Timezone::BritishWinter => FixedOffset::east_opt(0),
            Timezone::BritishSummer => FixedOffset::east_opt(3600),
        }
    }
}
