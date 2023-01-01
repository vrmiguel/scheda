use std::ops::Range;

use crate::error::{Error, Result};

mod month;

use crate::core::WellFormedRange;

/// Attempts to parse a single "atom"
pub trait AtomParse: Sized {
    fn parse_atom(val: &str) -> Option<Self>;
}

pub fn parse_schedule(schedule: &str) {}

fn nothing() {}

pub fn parse_range<T: AtomParse + WellFormedRange>(input: &str) -> Result<Range<T>> {
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
    use chrono::Month;
    use std::ops::Range;

    use super::parse_range;

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
