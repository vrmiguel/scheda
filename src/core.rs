use std::ops::Range;

use chrono::Month;
use smallvec::SmallVec;

pub trait WellFormedRange: Sized {
    fn is_well_formed(range: &Range<Self>) -> bool;
}

impl WellFormedRange for Month {
    fn is_well_formed(range: &Range<Self>) -> bool {
        range.start.number_from_month() < range.end.number_from_month()
    }
}

// pub struct Schedule<'a> {
//     /// The specification for months
//     month_spec: SmallVec<[MonthSpec; 4]>,
//     /// The command to be executed
//     command: &'a str,
// }
