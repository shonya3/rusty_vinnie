use poise::serenity_prelude::{CacheHttp, ChannelId};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum AppChannel {
    General,
    Poe,
    Poe2,
    LastEpoch,
    Dev,
}

impl AppChannel {
    pub async fn say(&self, ctx: impl CacheHttp, message: &str) {
        if let Err(err) = self.id().say(ctx, message).await {
            println!("Could not send message to channel: {err:#?}");
        };
    }
}

impl AppChannel {
    pub fn id(&self) -> ChannelId {
        match self {
            AppChannel::General => ChannelId::new(356012941083934722),
            AppChannel::Poe => ChannelId::new(356013349496029184),
            AppChannel::Poe2 => ChannelId::new(1399352084515520654),
            AppChannel::LastEpoch => ChannelId::new(1362313267879350363),
            AppChannel::Dev => ChannelId::new(841929108829372460),
        }
    }
}
