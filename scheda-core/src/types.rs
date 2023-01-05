use std::{num::NonZeroU8, ops::Range};

use crate::{as_u8::AsU8, Error, Result};

#[derive(Debug)]
pub struct Hour(NonZeroU8);

impl Hour {
    pub fn new(val: u8) -> Option<Self> {
        (val <= 24).then(|| {
            // Safety: val+1 can never be zero once val is unsigned
            Self(unsafe { NonZeroU8::new_unchecked(val + 1) })
        })
    }

    pub fn get(&self) -> u8 {
        // Cannot underflow
        self.0.get() - 1
    }
}

#[derive(Debug)]
pub struct Minute(NonZeroU8);

impl Minute {
    pub fn new(val: u8) -> Option<Self> {
        (val <= 60).then(|| {
            // Safety: val+1 can never be zero once val is unsigned
            Self(unsafe { NonZeroU8::new_unchecked(val + 1) })
        })
    }

    pub fn get(&self) -> u8 {
        // Cannot underflow
        self.0.get() - 1
    }
}

#[derive(Debug)]
pub struct MonthDay(u8);

impl MonthDay {
    pub fn from_u8(day: u8) -> Option<Self> {
        (day > 0 && day <= 31).then_some(Self(day))
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

pub enum DateTimePartKind {
    Month,
    Day,
    Hour,
    Minute,
    Every,
    Weekday,
}

impl DateTimePartKind {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            _x if value.eq_ignore_ascii_case("month") => Ok(Self::Month),
            _x if value.eq_ignore_ascii_case("day") => Ok(Self::Day),
            _x if value.eq_ignore_ascii_case("weekday") => Ok(Self::Weekday),
            _x if value.eq_ignore_ascii_case("hour") => Ok(Self::Hour),
            _x if value.eq_ignore_ascii_case("minute") => Ok(Self::Minute),
            _x if value.eq_ignore_ascii_case("every") => Ok(Self::Every),
            _ => Err(Error::UnknownDateTimePart(value.into())),
        }
    }
}

#[derive(Debug)]
/// Either a single date-time part or a range of them.
///
/// E.g. `when day 2` or `when weekday Thu to Fri`
pub enum DateTimePart<T> {
    Single(T),
    Range { starting: T, ending: T },
}

impl<T: AsU8> DateTimePart<T> {
    pub fn contains(&self, other: &T) -> bool {
        match self {
            DateTimePart::Single(single) => single.as_u8() == other.as_u8(),
            DateTimePart::Range { starting, ending } => {
                (starting.as_u8()..=ending.as_u8()).contains(&other.as_u8())
            }
        }
    }
}
