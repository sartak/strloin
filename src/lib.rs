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
//!   Time (mean ± σ):      1.258 s ±  0.084 s    [User: 0.073 s, System: 0.055 s]
//!   Range (min … max):    1.115 s …  1.364 s    10 runs
//!
//! Benchmark 2: strloin-slices
//!   Time (mean ± σ):     397.0 ms ±  35.8 ms    [User: 49.2 ms, System: 49.5 ms]
//!   Range (min … max):   345.7 ms … 445.5 ms    10 runs
//!
//! Benchmark 3: strloin-ranges
//!   Time (mean ± σ):     379.1 ms ±  41.1 ms    [User: 47.0 ms, System: 50.0 ms]
//!   Range (min … max):   310.6 ms … 432.3 ms    10 runs
//!
//! Summary
//!   'strloin-ranges' ran
//!     1.05 ± 0.15 times faster than 'strloin-slices'
//!     3.32 ± 0.42 times faster than 'always-clone'
//! ```
//!
//! ## Optional features
//!
//!- **`beef`** - Swap out the [`std::borrow::Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html) implementation for [`beef::lean::Cow`](https://docs.rs/beef/latest/beef/lean/type.Cow.html). The performance difference in my use case was just noise, but it may serve you better.

#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod cow;
mod ranges;
mod strloin;

pub use crate::cow::{Borrowed, Cow, Owned};
pub use crate::ranges::{collapse_ranges, Ranges};
pub use crate::strloin::Strloin;
