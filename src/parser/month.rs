use chrono::Month;
use num_traits::FromPrimitive;

use super::AtomParse;

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

impl MonthSpec {
    pub fn parse_month_idx(val: &str) -> Option<Month> {
        let idx: u32 = val.trim().parse().ok()?;

        Month::from_u32(idx)
    }

    pub fn parse_month_abbr(val: &str) -> Option<Month> {
        match val.trim() {
            x if x.eq_ignore_ascii_case("jan") => Some(Month::January),
            x if x.eq_ignore_ascii_case("feb") => Some(Month::February),
            x if x.eq_ignore_ascii_case("mar") => Some(Month::March),
            x if x.eq_ignore_ascii_case("apr") => Some(Month::April),
            x if x.eq_ignore_ascii_case("may") => Some(Month::May),
            x if x.eq_ignore_ascii_case("jun") => Some(Month::June),
            x if x.eq_ignore_ascii_case("jul") => Some(Month::July),
            x if x.eq_ignore_ascii_case("aug") => Some(Month::August),
            x if x.eq_ignore_ascii_case("sep") => Some(Month::September),
            x if x.eq_ignore_ascii_case("oct") => Some(Month::October),
            x if x.eq_ignore_ascii_case("nov") => Some(Month::November),
            x if x.eq_ignore_ascii_case("dec") => Some(Month::December),
            _ => None,
        }
    }

    pub fn parse_month(val: &str) -> Option<Month> {
        match val.trim() {
            x if x.eq_ignore_ascii_case("january") => Some(Month::January),
            x if x.eq_ignore_ascii_case("february") => Some(Month::February),
            x if x.eq_ignore_ascii_case("march") => Some(Month::March),
            x if x.eq_ignore_ascii_case("april") => Some(Month::April),
            x if x.eq_ignore_ascii_case("may") => Some(Month::May),
            x if x.eq_ignore_ascii_case("june") => Some(Month::June),
            x if x.eq_ignore_ascii_case("july") => Some(Month::July),
            x if x.eq_ignore_ascii_case("august") => Some(Month::August),
            x if x.eq_ignore_ascii_case("september") => Some(Month::September),
            x if x.eq_ignore_ascii_case("october") => Some(Month::October),
            x if x.eq_ignore_ascii_case("november") => Some(Month::November),
            x if x.eq_ignore_ascii_case("december") => Some(Month::December),
            _ => None,
        }
    }
}

impl AtomParse for Month {
    fn parse_atom(val: &str) -> Option<Self> {
        let trimmed = val.trim();

        MonthSpec::parse_month_idx(trimmed)
            .or_else(|| MonthSpec::parse_month_abbr(trimmed))
            .or_else(|| MonthSpec::parse_month(trimmed))
    }
}

#[cfg(test)]
mod tests {
    use chrono::Month;

    use super::AtomParse;

    #[test]
    fn parses_month_indexes() {
        let indexes = [
            ("1", Month::January),
            ("2", Month::February),
            ("3", Month::March),
            ("4", Month::April),
            ("5", Month::May),
            ("6", Month::June),
            ("7", Month::July),
            ("8", Month::August),
            ("9", Month::September),
            ("10", Month::October),
            ("11", Month::November),
            ("12", Month::December),
        ];

        for (to_parse, expected) in indexes {
            assert_eq!(Month::parse_atom(to_parse), Some(expected));
        }

        let bad_indexes = ["0", "120", "five", "12.2"];

        for idx in bad_indexes {
            assert!(Month::parse_atom(idx).is_none());
        }
    }

    #[test]
    fn parses_month_abbreviations() {
        let abbrvs = [
            ("Jan", Month::January),
            ("jan", Month::January),
            ("JAN", Month::January),
            ("FeB", Month::February),
            ("MAR", Month::March),
            ("apr", Month::April),
            ("mAY", Month::May),
            ("jun", Month::June),
            ("jul", Month::July),
            ("Aug", Month::August),
            ("Sep", Month::September),
            ("OcT", Month::October),
            ("nOV", Month::November),
            ("Dec", Month::December),
        ];

        for (to_parse, expected) in abbrvs {
            assert_eq!(Month::parse_atom(to_parse), Some(expected));
        }

        for abbrv in ["januar", "fb", "marc", "thursday"] {
            assert!(Month::parse_atom(abbrv).is_none())
        }
    }
}
