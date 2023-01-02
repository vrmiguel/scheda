use crate::core::{MonthDay, WellFormedRange};

use super::AtomParse;

impl AtomParse for MonthDay {
    fn parse_atom(val: &str) -> Option<Self> {
        let parsed = val.parse().ok()?;

        Self::from_u8(parsed)
    }
}

impl WellFormedRange for MonthDay {
    fn is_well_formed(range: &std::ops::Range<Self>) -> bool {
        range.start.as_u8() < range.end.as_u8()
    }
}