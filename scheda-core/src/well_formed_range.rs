use std::ops::Range;

use chrono::{Month, Weekday};

use crate::{as_u8::AsU8, types::MonthDay, Hour, Minute};

pub trait WellFormedRange: Sized {
    fn is_well_formed(range: &Range<Self>) -> bool;
}

impl WellFormedRange for Month {
    fn is_well_formed(range: &Range<Self>) -> bool {
        range.start.number_from_month() < range.end.number_from_month()
    }
}

impl WellFormedRange for MonthDay {
    fn is_well_formed(range: &std::ops::Range<Self>) -> bool {
        range.start.as_u8() < range.end.as_u8()
    }
}

impl WellFormedRange for Hour {
    fn is_well_formed(range: &std::ops::Range<Self>) -> bool {
        range.start.as_u8() < range.end.as_u8()
    }
}

impl WellFormedRange for Minute {
    fn is_well_formed(range: &std::ops::Range<Self>) -> bool {
        range.start.as_u8() < range.end.as_u8()
    }
}

impl WellFormedRange for Weekday {
    fn is_well_formed(range: &std::ops::Range<Self>) -> bool {
        range.start.num_days_from_sunday() < range.end.num_days_from_sunday()
    }
}
