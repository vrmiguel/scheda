use scheda_core::chrono::Weekday;

use super::AtomParse;

impl AtomParse for Weekday {
    fn parse_atom(val: &str) -> Option<Self> {
        let trimmed = val.trim();

        parse_weekday_idx(trimmed)
            .or_else(|| parse_weekday_abbr(trimmed))
            .or_else(|| parse_weekday(trimmed))
    }
}

pub fn parse_weekday_idx(val: &str) -> Option<Weekday> {
    let idx: u32 = val.parse().ok()?;

    match idx {
        0 => Some(Weekday::Sun),
        1 => Some(Weekday::Mon),
        2 => Some(Weekday::Tue),
        3 => Some(Weekday::Wed),
        4 => Some(Weekday::Thu),
        5 => Some(Weekday::Fri),
        6 => Some(Weekday::Sat),
        _ => None,
    }
}

fn parse_weekday_abbr(val: &str) -> Option<Weekday> {
    match val {
        x if x.eq_ignore_ascii_case("mon") => Some(Weekday::Mon),
        x if x.eq_ignore_ascii_case("tue") => Some(Weekday::Tue),
        x if x.eq_ignore_ascii_case("wed") => Some(Weekday::Wed),
        x if x.eq_ignore_ascii_case("thu") => Some(Weekday::Thu),
        x if x.eq_ignore_ascii_case("fri") => Some(Weekday::Fri),
        x if x.eq_ignore_ascii_case("sat") => Some(Weekday::Sat),
        x if x.eq_ignore_ascii_case("sun") => Some(Weekday::Sun),
        _ => None,
    }
}

fn parse_weekday(val: &str) -> Option<Weekday> {
    match val {
        x if x.eq_ignore_ascii_case("monday") => Some(Weekday::Mon),
        x if x.eq_ignore_ascii_case("tuesday") => Some(Weekday::Tue),
        x if x.eq_ignore_ascii_case("wednesday") => Some(Weekday::Wed),
        x if x.eq_ignore_ascii_case("thursday") => Some(Weekday::Thu),
        x if x.eq_ignore_ascii_case("friday") => Some(Weekday::Fri),
        x if x.eq_ignore_ascii_case("saturday") => Some(Weekday::Sat),
        x if x.eq_ignore_ascii_case("sunday") => Some(Weekday::Sun),
        _ => None,
    }
}
