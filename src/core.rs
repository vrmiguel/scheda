use std::ops::Range;

use chrono::Month;
use smallvec::SmallVec;

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct MonthDay(u8);

impl MonthDay {
    pub fn from_u8(day: u8) -> Option<Self> {
        (day > 0 && day <= 31).then_some(Self(day))
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

pub trait WellFormedRange: Sized {
    fn is_well_formed(range: &Range<Self>) -> bool;
}

impl WellFormedRange for Month {
    fn is_well_formed(range: &Range<Self>) -> bool {
        range.start.number_from_month() < range.end.number_from_month()
    }
}

/// Specification for months.
///
/// Months themselves might be described by:
/// * their month numbers (1 to 12)
/// * their full names in EN locale (e.g. `"January"`)
/// * their abbreviated names in `strftime` EN locale
///     * `Jan`, `Feb`, `Mar`, `Apr`, `May`, `Jun`, `Jul`, `Aug`, `Sep`, `Oct`, `Nov` and `Dec`
///
/// # Syntax
///
/// ## Single months
///
/// ```no-rust
/// when month Dec
/// when month December
/// when month 10
/// ```
///
/// ## Ranges
///
/// ```no-rust
/// when month Jan to Apr
/// when month June to October
/// when month 1 to 9
/// when month 3 to Dec
/// ```
pub enum MonthSpec {
    Single(Month),
    Range { starting: Month, ending: Month },
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
    pub fn from_str(value: &str) -> Result<Self> {
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

    /// The identifier of this date-time part kind as found in the schedule syntax.
    ///
    /// E.g. `"month"` in `"when month December"`.
    pub fn identifier(&self) -> &str {
        match self {
            DateTimePartKind::Month => "month",
            DateTimePartKind::Day => "day",
            DateTimePartKind::Hour => "hour",
            DateTimePartKind::Minute => "minute",
            DateTimePartKind::Every => "every",
        }
    }
}

#[derive(Debug)]
pub enum DateTimePart<T> {
    Single(T),
    Range { starting: T, ending: T },
}

pub enum Spec {
    Month(MonthSpec),
}

#[derive(Debug)]
pub struct Schedule {
    /// The specification for month days
    pub(crate) day_spec: SmallVec<[DateTimePart<MonthDay>; 4]>,
    /// The specification for months
    pub(crate) month_spec: SmallVec<[DateTimePart<Month>; 4]>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            month_spec: SmallVec::new(),
            day_spec: SmallVec::new(),
        }
    }
}
