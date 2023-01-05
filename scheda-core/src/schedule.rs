use std::fmt::Debug;

use chrono::{DateTime, Datelike, Month, NaiveDateTime, TimeZone, Timelike, Weekday};
use num_traits::FromPrimitive;
use smallvec::SmallVec;

use crate::{AsU8, DateTimePart, Hour, Minute, MonthDay};

#[derive(Debug)]
pub struct Schedule {
    /// The specification for month days
    pub hour_spec: SmallVec<[DateTimePart<Hour>; 2]>,
    /// The specification for month days
    pub minute_spec: SmallVec<[DateTimePart<Minute>; 2]>,
    /// The specification for month days
    pub day_spec: SmallVec<[DateTimePart<MonthDay>; 4]>,
    /// The specification for months
    pub month_spec: SmallVec<[DateTimePart<Month>; 4]>,
    /// The specification for months
    pub weekday_spec: SmallVec<[DateTimePart<Weekday>; 4]>,
    // TODO: add timezone
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
fn contains_element<'a, T: AsU8 + Debug + 'a>(parts: &[DateTimePart<T>], element: &T) -> bool {
    if parts.is_empty() {
        // Wild-card
        return true;
    }

    parts.iter().any(|part| part.contains(&element))
}

pub struct DateTimePartExtractor<Tz: TimeZone> {
    date_time: DateTime<Tz>,
    // naive_date_time: NaiveDateTime
}

impl<Tz: TimeZone> DateTimePartExtractor<Tz> {
    pub fn new(date_time: DateTime<Tz>) -> Self {
        Self {
            // naive_date_time: date_time.naive_local(),
            date_time,
        }
    }

    pub fn hour(&self) -> Hour {
        Hour::new(self.date_time.time().hour() as u8).unwrap()
    }

    pub fn minute(&self) -> Minute {
        Minute::new(self.date_time.time().minute() as u8).unwrap()
    }

    pub fn month(&self) -> Month {
        Month::from_u32(self.date_time.month()).unwrap()
    }

    pub fn month_day(&self) -> MonthDay {
        MonthDay::from_u8(self.date_time.day() as u8).unwrap()
    }

    pub fn weekday(&self) -> Weekday {
        self.date_time.weekday()
    }
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            month_spec: SmallVec::new(),
            day_spec: SmallVec::new(),
            weekday_spec: SmallVec::new(),
            hour_spec: SmallVec::new(),
            minute_spec: SmallVec::new(),
        }
    }

    fn contains_hour(&self, hour: Hour) -> bool {
        contains_element(self.hour_spec.as_slice(), &hour)
    }

    fn contains_minute(&self, minute: Minute) -> bool {
        contains_element(self.minute_spec.as_slice(), &minute)
    }

    fn contains_day(&self, day: MonthDay) -> bool {
        contains_element(self.day_spec.as_slice(), &day)
    }

    fn contains_month(&self, month: Month) -> bool {
        contains_element(self.month_spec.as_slice(), &month)
    }

    fn contains_weekday(&self, weekday: Weekday) -> bool {
        contains_element(self.weekday_spec.as_slice(), &weekday)
    }

    pub fn matches<TZ: TimeZone>(&self, date_time: DateTime<TZ>) -> bool {
        let extractor = DateTimePartExtractor::new(date_time);

        self.contains_month(extractor.month())
            && self.contains_day(extractor.month_day())
            && self.contains_hour(extractor.hour())
            && self.contains_minute(extractor.minute())
            && self.contains_weekday(extractor.weekday())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
    use scheda_parser::Parser;

    fn from_ymdhm(year: i32, month: u32, day: u32, hour: u32, min: u32) -> DateTime<Utc> {
        DateTime::from_utc(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(year, month, day).unwrap(),
                NaiveTime::from_hms_opt(hour, min, 0).unwrap(),
            ),
            Utc,
        )
    }

    fn assert_matches(date_time: DateTime<Utc>, schedule: &str) {
        let schedule = Parser::new(schedule).parse_schedule().unwrap();

        assert!(schedule.matches(date_time));
    }

    fn assert_no_match(date_time: DateTime<Utc>, schedule: &str) {
        let schedule = Parser::new(schedule).parse_schedule().unwrap();

        assert_eq!(schedule.matches(date_time), false);
    }

    #[test]
    fn matches_dates_to_schedule() {
        // Tuesday, 22/05/2001 20h00
        let d05_22_2001 = from_ymdhm(2001, 5, 22, 20, 00);

        assert_matches(d05_22_2001, "when weekday tuesday");
        assert_matches(d05_22_2001, "when weekday monday to wednesday");
        assert_no_match(d05_22_2001, "when weekday wednesday or saturday");
        assert_no_match(d05_22_2001, "when weekday wednesday to friday");

        assert_matches(
            d05_22_2001,
            "when month 5, day 19 to 22, hour 20, weekday tuesday",
        );
        assert_matches(
            d05_22_2001,
            "when month 5 to 7, day 19 to 22, hour 18 to 21, weekday tuesday",
        );
        assert_no_match(
            d05_22_2001,
            "when month 6 to 11, day 19 to 22, hour 18 to 21, weekday tuesday",
        );
        assert_no_match(
            d05_22_2001,
            "when month 5, day 19 to 21, hour 18 to 21, weekday tuesday",
        );
        assert_no_match(
            d05_22_2001,
            "when month 5, day 19 to 22, hour 18 to 19, weekday tuesday",
        );
        assert_matches(
            d05_22_2001,
            "when month 5, day 19 to 22, hour 20, weekday tuesday",
        );
    }
}
