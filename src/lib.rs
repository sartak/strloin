//! `strloin` gives you copy-on-write (cow) slices of a string. If the provided ranges form a
//! single contiguous region, then you'll get back a borrowed slice of the string. Otherwise,
//! you'll get back an owned concatenation of each range.
//!
//! ```rust
//! use strloin::Strloin;
//!
//! let strloin = Strloin::new("hello world");
//!
//! assert_eq!(strloin.from_ranges(&[0..5]), "hello"); // borrowed
//! assert_eq!(strloin.from_ranges(&[0..5, 5..11]), "hello world"); // borrowed
//! assert_eq!(strloin.from_ranges(&[0..5, 6..11]), "helloworld"); // owned
//! ```
//!
//! Note that this crate is intended for cases where borrowing is far more common than cloning. If
//! cloning is common, then it's likely that the performance overhead, much less the cognitive
//! overhead, is too expensive and you should consider unconditionally cloning. Your mileage will
//! vary. But, on a real-world text parser where 85% of `from_ranges` resulted in a borrow,
//! switching from always cloning to conditionally cloning with strloin had the following impact:
//!
//! ```text
//! Benchmark 1: always-clone
//!   Time (mean ± σ):      1.259 s ±  0.089 s    [User: 0.062 s, System: 0.063 s]
//!   Range (min … max):    1.082 s …  1.367 s    10 runs
//!
//! Benchmark 2: strloin-slices
//!   Time (mean ± σ):     394.7 ms ±  40.0 ms    [User: 49.5 ms, System: 50.7 ms]
//!   Range (min … max):   310.7 ms … 452.0 ms    10 runs
//!
//! Benchmark 3: strloin-ranges
//!   Time (mean ± σ):     376.5 ms ±  36.1 ms    [User: 45.5 ms, System: 56.5 ms]
//!   Range (min … max):   324.7 ms … 441.2 ms    10 runs
//!
//! Summary
//!   'strloin-ranges' ran
//!     1.05 ± 0.15 times faster than 'strloin-slices'
//!     3.34 ± 0.40 times faster than 'always-clone'
//! ```

#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod cow;
mod ranges;
mod strloin;

pub use crate::cow::{Borrowed, Cow, Owned};
pub use crate::ranges::{collapse_ranges, Ranges};
pub use crate::strloin::Strloin;
