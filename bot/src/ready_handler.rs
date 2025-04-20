use crate::{
    channel::AppChannel,
    last_epoch::{watch_lastepoch, Subforum as LastEpochSubforum},
    poe_newsletter::spin_news_loop,
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
        watch_lastepoch(ctx, LastEpochSubforum::Announcements),
        watch_lastepoch(ctx, LastEpochSubforum::DeveloperBlogs),
        watch_lastepoch(ctx, LastEpochSubforum::News),
        watch_lastepoch(ctx, LastEpochSubforum::PatchNotes),
        spin_teasers_loop(ctx, data, &[]),
        spin_news_loop(ctx, &WebsiteLanguage::En, &Subforum::News, offset),
        spin_news_loop(ctx, &WebsiteLanguage::Ru, &Subforum::News, offset),
        spin_news_loop(ctx, &WebsiteLanguage::En, &Subforum::PatchNotes, offset),
        spin_news_loop(ctx, &WebsiteLanguage::Ru, &Subforum::PatchNotes, offset),
        spin_news_loop(
            ctx,
            &WebsiteLanguage::En,
            &Subforum::EarlyAccessPatchNotesEn,
            offset
        ),
        spin_news_loop(
            ctx,
            &WebsiteLanguage::Ru,
            &Subforum::EarlyAccessPatchNotesRu,
            offset
        ),
        spin_news_loop(
            ctx,
            &WebsiteLanguage::En,
            &Subforum::EarlyAccessAnnouncementsEn,
            offset
        ),
        spin_news_loop(
            ctx,
            &WebsiteLanguage::Ru,
            &Subforum::EarlyAccessAnnouncementsRu,
            offset
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
