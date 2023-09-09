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
//! Benchmark 1: strloin
//!   Time (mean ± σ):     324.4 ms ±  38.1 ms    [User: 48.1 ms, System: 51.5 ms]
//!   Range (min … max):   258.0 ms … 370.9 ms    10 runs
//!
//! Benchmark 2: always-clone
//!   Time (mean ± σ):     688.8 ms ±  69.4 ms    [User: 53.4 ms, System: 66.3 ms]
//!   Range (min … max):   577.5 ms … 774.4 ms    10 runs
//!
//! Summary
//!   'strloin' ran
//!     2.12 ± 0.33 times faster than 'always-clone'
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
pub use crate::ranges::collapse_ranges;
pub use crate::strloin::Strloin;
