//! # Scheduling syntax
//!
//! ## Months
//!
//! Months themselves might be described by:
//! * their month numbers (1 to 12)
//! * their full names in EN locale (e.g. `"January"`)
//! * their abbreviated names in `strftime` EN locale
//!     * `Jan`, `Feb`, `Mar`, `Apr`, `May`, `Jun`, `Jul`, `Aug`, `Sep`, `Oct`, `Nov` and `Dec`
//!
//! # Syntax
//!
//! ## Single months
//!
//! ```no-rust
//! when month Dec
//! when month December
//! when month 10
//! ```
//!
//! ## Ranges
//!
//! ```no-rust
//! when month Jan to Apr
//! when month June to October
//! when month 1 to 9
//! when month 3 to Dec
//! ```

mod as_u8;
mod error;
mod schedule;
mod types;
mod well_formed_range;

pub use as_u8::AsU8;
pub use chrono;
pub use error::{Error, Result};
pub use schedule::Schedule;
pub use types::*;
pub use well_formed_range::WellFormedRange;

// TODO: Revisit ZeroToN to check if we want to use it
// struct ZeroToN<const N: usize>(NonZeroU8);

// impl <const N: usize> ZeroToN<N> {
//     pub fn new(val: u8) -> Option<Self> {
//         (val <= (N as u8)).then(|| {
//             // Safety: val+1 can never be zero once val is unsigned
//             Self(unsafe { NonZeroU8::new_unchecked(val + 1) })
//         })
//     }
// }
