use crate::{
    channel::AppChannel,
    newsletter,
    status::{get_kroiya_status, watch_status},
    Data,
};
use chrono::FixedOffset;
use futures::future::join_all;
use last_epoch_forum::Subforum as LastEpochSubforum;
use poe_forum::{Subforum, WebsiteLanguage};
use poe_teasers::TeasersForumThread;
use poise::serenity_prelude::{self as serenity};

pub async fn handle_ready(ctx: &serenity::Context, data: &Data) {
    println!("Bot is ready");

    set_watchers(ctx, data).await;
}

async fn set_watchers(ctx: &serenity::Context, data: &Data) {
    let teasers_3_27 = crate::poe_teasers::watch_teasers_threads(
        ctx,
        data,
        &[
            TeasersForumThread::Poe1_3_27Ru,
            TeasersForumThread::Poe1_3_27En,
        ],
    );

    let diablo = newsletter::start_news_feed(ctx, AppChannel::Diablo, async || {
        diablo::fetch_posts().await.map(|posts| {
            posts
                .into_iter()
                .filter(|post| !post.category.is_console_related())
                .collect()
        })
    });

    let last_epoch = join_all(
        [
            LastEpochSubforum::Announcements,
            LastEpochSubforum::DeveloperBlogs,
            LastEpochSubforum::News,
            LastEpochSubforum::PatchNotes,
        ]
        .into_iter()
        .map(async |subforum: LastEpochSubforum| {
            newsletter::start_news_feed(ctx, AppChannel::LastEpoch, async || {
                last_epoch_forum::fetch_subforum_threads_list(subforum).await
            })
            .await;
        }),
    );

    let poe1 = join_all(
        [
            (WebsiteLanguage::En, Subforum::News),
            (WebsiteLanguage::Ru, Subforum::News),
            (WebsiteLanguage::En, Subforum::PatchNotes),
            (WebsiteLanguage::Ru, Subforum::PatchNotes),
        ]
        .into_iter()
        .map(async |(lang, subforum)| {
            newsletter::start_news_feed(ctx, AppChannel::Poe, async || {
                poe_forum::fetch_subforum_threads_list(
                    lang,
                    subforum,
                    Timezone::BritishWinter.offset().as_ref(),
                )
                .await
            })
            .await
        }),
    );

    let poe2 = join_all(
        [
            (WebsiteLanguage::En, Subforum::EarlyAccessPatchNotesEn),
            (WebsiteLanguage::Ru, Subforum::EarlyAccessPatchNotesRu),
            (WebsiteLanguage::En, Subforum::EarlyAccessAnnouncementsEn),
            (WebsiteLanguage::Ru, Subforum::EarlyAccessAnnouncementsRu),
        ]
        .into_iter()
        .map(async |(lang, subforum)| {
            newsletter::start_news_feed(ctx, AppChannel::Poe2, async || {
                poe_forum::fetch_subforum_threads_list(
                    lang,
                    subforum,
                    Timezone::BritishWinter.offset().as_ref(),
                )
                .await
            })
            .await
        }),
    );

    tokio::join!(
        teasers_3_27,
        watch_status(
            || get_kroiya_status(ctx),
            || AppChannel::General.say(ctx, ":rabbit: пришел"),
            || AppChannel::General.say(ctx, ":rabbit: ушел"),
        ),
        last_epoch,
        poe1,
        poe2,
        diablo,
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
