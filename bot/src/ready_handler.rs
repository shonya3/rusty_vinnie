use std::time::Duration;

use crate::{
    challenges::start_daily_summarizer,
    channel::AppChannel,
    interval::Timezone,
    newsletter::{
        diablo::DiabloNewsletter, last_epoch::LastEpochNewsletter, poe::PoeNewsletter, Newsletter,
    },
    status::{get_kroiya_status, watch_status},
    stream_announcer::{self, Offset},
    Data,
};
use futures::future::join_all;
use last_epoch_forum::Subforum as LastEpochSubforum;
use poe_forum::{Subforum, WebsiteLanguage};
use poe_teasers::TeasersForumThread;
use poise::serenity_prelude::{self as serenity};
use rand::seq::IndexedRandom;

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

    let diablo_newsletter = DiabloNewsletter;
    let diablo = diablo_newsletter.start(ctx, AppChannel::Diablo);

    let last_epoch_newsletter = LastEpochNewsletter::new(vec![
        LastEpochSubforum::Announcements,
        LastEpochSubforum::DeveloperBlogs,
        LastEpochSubforum::News,
        LastEpochSubforum::PatchNotes,
    ]);
    let last_epoch = last_epoch_newsletter.start(ctx, AppChannel::LastEpoch);

    let poe1_newsletter = PoeNewsletter::new(
        vec![
            (WebsiteLanguage::En, Subforum::News),
            (WebsiteLanguage::Ru, Subforum::News),
            (WebsiteLanguage::En, Subforum::PatchNotes),
            (WebsiteLanguage::Ru, Subforum::PatchNotes),
        ],
        Timezone::Moscow,
    );
    let poe1 = poe1_newsletter.start(ctx, AppChannel::Poe);

    let poe2_newsletter = PoeNewsletter::new(
        vec![
            (WebsiteLanguage::En, Subforum::EarlyAccessPatchNotesEn),
            (WebsiteLanguage::Ru, Subforum::EarlyAccessPatchNotesRu),
            (WebsiteLanguage::En, Subforum::EarlyAccessAnnouncementsEn),
            (WebsiteLanguage::Ru, Subforum::EarlyAccessAnnouncementsRu),
        ],
        Timezone::Moscow,
    );
    let poe2 = poe2_newsletter.start(ctx, AppChannel::Poe2);

    let challenge_summarizer = start_daily_summarizer(ctx);

    let presence_updater = async move {
        let mut interval = tokio::time::interval(Duration::from_mins(1));
        loop {
            interval.tick().await;
            stream_announcer::update_presence(ctx);
        }
    };

    let e = || {
        [
            "⏰", "🚨", "🐸", "🔥", "🎮", "✨", "🎉", "🚀", "🌟", "🔴", "💥", "⚡", "🌈", "🐭",
            "🤓", "😎", "🦀",
        ]
        .choose(&mut rand::rng())
        .unwrap()
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
                    let e1 = format!("{}{}{}", e(), e(), e());
                    let e2 = format!("{}{}{}", e(), e(), e());
                    let msg = format!("{e1} Stream starts in {} {e2}", offset.label());
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
