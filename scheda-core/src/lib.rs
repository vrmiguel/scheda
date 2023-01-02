mod error;
mod well_formed_range;
mod types;

pub use well_formed_range::WellFormedRange;
pub use error::{Error, Result};
pub use chrono;
pub use types::*;

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

