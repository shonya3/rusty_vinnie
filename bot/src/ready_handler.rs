use std::time::Duration;

use crate::{
    challenges::start_daily_summarizer,
    channel::AppChannel,
    newsletter,
    status::{get_kroiya_status, watch_status},
    stream_announcer::{self, Offset},
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
    let teasers = crate::poe_teasers::watch_teasers_threads(
        ctx,
        data,
        &[
            TeasersForumThread::Poe2_05(poe_teasers::Lang::En),
            TeasersForumThread::Poe2_05(poe_teasers::Lang::Ru),
        ],
        AppChannel::Poe2,
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

    let timezone_offset = Timezone::Moscow.offset();

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
                poe_forum::fetch_subforum_threads_list(lang, subforum, timezone_offset.as_ref())
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
                poe_forum::fetch_subforum_threads_list(lang, subforum, timezone_offset.as_ref())
                    .await
            })
            .await
        }),
    );

    let challenge_summarizer = start_daily_summarizer(ctx);

    let presence_updater = async move {
        let mut interval = tokio::time::interval(Duration::from_mins(1));
        loop {
            interval.tick().await;
            stream_announcer::update_presence(ctx);
        }
    };

    let stream_announcer = join_all(
        [
            Offset::Hours(48),
            Offset::Hours(24),
            Offset::Hours(12),
            Offset::Hours(10),
            Offset::Hours(8),
            Offset::Hours(6),
            Offset::Hours(5),
            Offset::Hours(4),
            Offset::Hours(3),
            Offset::Hours(2),
            Offset::Hours(1),
            Offset::Minutes(45),
            Offset::Minutes(30),
            Offset::Minutes(15),
            Offset::Minutes(10),
            Offset::Minutes(5),
            Offset::Minutes(2),
            Offset::Minutes(1),
        ]
        .into_iter()
        .filter(|o| o.is_upcoming())
        .map(|offset| async move {
            offset
                .schedule(move || async move {
                    let msg = format!("⏰ Stream starts in {}!", offset.label());
                    AppChannel::Poe2.say(&ctx, &msg).await;
                })
                .await;
        }),
    );

    tokio::join!(
        stream_announcer,
        presence_updater,
        watch_status(
            || get_kroiya_status(ctx),
            || AppChannel::General.say(ctx, ":rabbit: пришел"),
            || AppChannel::General.say(ctx, ":rabbit: ушел"),
        ),
        teasers,
        last_epoch,
        poe1,
        poe2,
        diablo,
        challenge_summarizer,
    );
}

#[allow(dead_code)]
pub enum Timezone {
    BritishWinter,
    BritishSummer,
    Moscow,
}

impl Timezone {
    pub fn offset(&self) -> Option<FixedOffset> {
        match self {
            Timezone::BritishWinter => FixedOffset::east_opt(0),
            Timezone::BritishSummer => FixedOffset::east_opt(3600),
            Timezone::Moscow => FixedOffset::east_opt(3600 * 3),
        }
    }
}
