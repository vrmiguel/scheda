use crate::core::{Hour, Minute, WellFormedRange};

use super::AtomParse;

fn parse_ranged(val: &str, max: u8) -> Option<u8> {
    let idx = val.parse().ok()?;

    (idx <= max).then_some(idx)
}

impl AtomParse for Hour {
    fn parse_atom(val: &str) -> Option<Self> {
        let trimmed = val.trim();

        parse_ranged(trimmed, 24).and_then(Hour::new)
    }
}

impl WellFormedRange for Hour {
    fn is_well_formed(range: &std::ops::Range<Self>) -> bool {
        range.start.as_u8() < range.end.as_u8()
    }
}

impl AtomParse for Minute {
    fn parse_atom(val: &str) -> Option<Self> {
        let trimmed = val.trim();

        parse_ranged(trimmed, 60).and_then(Minute::new)
    }
}

impl WellFormedRange for Minute {
    fn is_well_formed(range: &std::ops::Range<Self>) -> bool {
        range.start.as_u8() < range.end.as_u8()
    }
}
