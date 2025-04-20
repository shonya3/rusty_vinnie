use poise::serenity_prelude::ChannelId;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum AppChannel {
    General,
    Poe,
    LastEpoch,
    Dev,
}

impl AppChannel {
    pub fn id(&self) -> ChannelId {
        match self {
            AppChannel::General => ChannelId::new(356012941083934722),
            AppChannel::Poe => ChannelId::new(356013349496029184),
            AppChannel::LastEpoch => ChannelId::new(1362313267879350363),
            AppChannel::Dev => ChannelId::new(841929108829372460),
        }
    }
}
