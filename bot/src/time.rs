use chrono::{DateTime, FixedOffset, Utc};
pub const TIMEZONE: Timezone = Timezone::Moscow;

#[allow(unused)]
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

pub fn fmt(utc: DateTime<Utc>, with_timezone: bool) -> String {
    let format = "%d.%m  %H:%M";

    let s = match with_timezone {
        true => utc
            .with_timezone(&TIMEZONE.offset().unwrap())
            .format(format),
        false => utc.format(format),
    };

    format!("{s}")
}
