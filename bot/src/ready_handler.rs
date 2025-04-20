use crate::{
    channel::AppChannel,
    last_epoch::{self, Subforum as LastEpochSubforum},
    poe_newsletter,
    status::{get_kroiya_status, watch_status},
    teasers::spin_teasers_loop,
    Data,
};
use chrono::FixedOffset;
use fresh_news::{Subforum, WebsiteLanguage};
use poise::serenity_prelude::{self as serenity};

pub async fn handle_ready(ctx: &serenity::Context, data: &Data) {
    println!("Bot is ready");
    let say = |message: &'static str| async move {
        if let Err(err) = AppChannel::General.id().say(ctx, message).await {
            println!("Could not send message to channel: {err:#?}");
        };
    };

    let offset = Timezone::BritishSummer.offset();

    tokio::join!(
        watch_status(
            || get_kroiya_status(ctx),
            || say(":rabbit: пришел"),
            || say(":rabbit: ушел"),
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
            offset,
        ),
        spin_teasers_loop(ctx, data, &[]),
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
