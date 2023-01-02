use scheda_core::MonthDay;

use super::AtomParse;

impl AtomParse for MonthDay {
    fn parse_atom(val: &str) -> Option<Self> {
        let parsed = val.parse().ok()?;

        Self::from_u8(parsed)
    }
}
