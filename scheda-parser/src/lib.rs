use std::ops::Range;

mod day;
mod hour_and_minutes;
mod month;
mod weekday;

use scheda_core::{DateTimePart, DateTimePartKind, Error, Result, Schedule, WellFormedRange};

/// Attempts to parse a single "atom"
pub trait AtomParse: Sized {
    fn parse_atom(val: &str) -> Option<Self>;
}

pub struct Parser<'a> {
    input: &'a str,
    schedule: Schedule,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Self {
            input: input.trim(),
            schedule: Schedule::new(),
        }
    }
}

impl<'a> Parser<'a> {
    fn eat_when(&mut self) -> Result<()> {
        let (before_when, remaining) = self
            .input
            .split_once("when")
            .ok_or(Error::MissingWhenStmt)?;

        // TODO: treat this with the error enum
        assert!(before_when.is_empty());

        self.input = remaining;

        Ok(())
    }

    fn parse_kind<'b>(&'a self, value: &'b str) -> Result<(DateTimePartKind, &'b str)> {
        // TODO: treat error
        let (identifier, rest) = value.split_once(' ').ok_or(Error::InvalidSyntax(
            "expected a date-time part identifier, such as `month` or `hour`".into(),
        ))?;

        DateTimePartKind::parse(identifier).map(|kind| (kind, rest))
    }

    fn parse_atom_or_range<T: AtomParse + WellFormedRange>(
        &mut self,
        value: &str,
    ) -> Result<DateTimePart<T>> {
        match parse_range(value) {
            Ok(range) => Ok(DateTimePart::Range {
                starting: range.start,
                ending: range.end,
            }),
            Err(_) => {
                // TODO: why error handling so bad :C
                let atom =
                    T::parse_atom(value).ok_or(Error::InvalidSyntax(value.to_string().into()))?;
                Ok(DateTimePart::Single(atom))
            }
        }
    }

    fn parse_spec(&mut self, value: &str) -> Result<()> {
        let (kind, rest) = self.parse_kind(value)?;

        for item in rest.split(" or ") {
            match kind {
                DateTimePartKind::Month => {
                    let part = self.parse_atom_or_range(item)?;
                    self.schedule.month_spec.push(part);
                }
                DateTimePartKind::Day => {
                    let part = self.parse_atom_or_range(item)?;
                    self.schedule.day_spec.push(part);
                }
                DateTimePartKind::Weekday => {
                    let part = self.parse_atom_or_range(item)?;
                    self.schedule.weekday_spec.push(part);
                }
                DateTimePartKind::Hour => {
                    let part = self.parse_atom_or_range(item)?;
                    self.schedule.hour_spec.push(part);
                }
                DateTimePartKind::Minute => {
                    let part = self.parse_atom_or_range(item)?;
                    self.schedule.minute_spec.push(part);
                }
                DateTimePartKind::Every => todo!(),
            }
        }

        Ok(())
    }

    pub fn parse_schedule(mut self) -> Result<Schedule> {
        // Eat the leading `when` statement
        self.eat_when()?;

        // Date-time parts are separated by commas
        for decl in self.input.split(',') {
            self.parse_spec(decl.trim())?;
        }

        Ok(self.schedule)
    }
}

fn nothing() {}

fn parse_range<T: AtomParse + WellFormedRange>(input: &str) -> Result<Range<T>> {
    fn parse_inner<T: AtomParse + WellFormedRange>(input: &str) -> Option<Range<T>> {
        let mut parts = input.trim().split(' ');

        let start = T::parse_atom(parts.next()?)?;

        // Eat "to"
        (parts.next()? == "to").then(nothing)?;

        let end = T::parse_atom(parts.next()?)?;

        Some(Range { start, end })
    }

    let range = parse_inner(input).ok_or(Error::MalformedRange("incorrect syntax"))?;

    // Ensure this range is well-formed
    T::is_well_formed(&range)
        .then(nothing)
        .ok_or(Error::MalformedRange(
            "range's starting point is bigger than its end",
        ))?;

    Ok(range)
}

#[cfg(test)]
mod tests {
    use scheda_core::chrono::Month;
    use std::ops::Range;

    use super::{parse_range, Parser};

    #[test]
    fn parses_month_specs() {
        Parser::new("when month feb to mar")
            .parse_schedule()
            .unwrap();
        Parser::new("when month 10").parse_schedule().unwrap();
    }

    #[test]
    fn parses_month_ranges() {
        assert_eq!(
            parse_range("Jan to Mar").unwrap(),
            Range {
                start: Month::January,
                end: Month::March
            }
        );
        assert_eq!(
            parse_range("february to 10").unwrap(),
            Range {
                start: Month::February,
                end: Month::October
            }
        );
        assert_eq!(
            parse_range("Nov to Dec").unwrap(),
            Range {
                start: Month::November,
                end: Month::December
            }
        );
        assert_eq!(
            parse_range("2 to 10").unwrap(),
            Range {
                start: Month::February,
                end: Month::October
            }
        );

        // Bad: beginning bigger than ending
        assert!(parse_range::<Month>("10 to 5").is_err());
        assert!(parse_range::<Month>("Dec to Feb").is_err());

        // Bad: single-point ranges are not allowed
        assert!(parse_range::<Month>("Dec to Dec").is_err());
        assert!(parse_range::<Month>("february to Feb").is_err());

        // Bad: non-sense month values
        assert!(parse_range::<Month>("Dec to Tuesday").is_err());
        assert!(parse_range::<Month>("13 to 19").is_err());
    }
}
