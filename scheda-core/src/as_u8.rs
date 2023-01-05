use chrono::{Month, Weekday};

use crate::{Hour, Minute, MonthDay};

pub trait AsU8: Sized {
    fn as_u8(&self) -> u8;
}

impl AsU8 for Hour {
    fn as_u8(&self) -> u8 {
        self.get()
    }
}

impl AsU8 for Minute {
    fn as_u8(&self) -> u8 {
        self.get()
    }
}

impl AsU8 for Month {
    fn as_u8(&self) -> u8 {
        self.number_from_month() as u8
    }
}

impl AsU8 for MonthDay {
    fn as_u8(&self) -> u8 {
        self.get()
    }
}

impl AsU8 for Weekday {
    fn as_u8(&self) -> u8 {
        self.num_days_from_sunday() as u8
    }
}
