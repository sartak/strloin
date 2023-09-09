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

#[cfg(feature = "beef")]
pub use beef::lean::Cow;
#[cfg(feature = "beef")]
#[allow(non_snake_case)]
pub fn Borrowed<'a>(val: &'a str) -> beef::lean::Cow<'a, str> {
    Cow::borrowed(val)
}
#[cfg(feature = "beef")]
#[allow(non_snake_case)]
pub fn Owned<'a>(val: String) -> beef::lean::Cow<'a, str> {
    Cow::owned(val)
}

#[cfg(not(feature = "beef"))]
pub use std::borrow::Cow::{self, Borrowed, Owned};

use std::ops::Range;

pub struct Strloin<'a> {
    pub source: &'a str,
}

fn collapse_ranges(ranges: &[Range<usize>]) -> Option<Range<usize>> {
    let mut rs = ranges.iter();
    let Some(first) = rs.next() else {
        return Some(Range { start: 0, end: 0 });
    };
    let start = first.start;
    let mut end = first.end;

    if start > end {
        return None;
    }

    for r in rs {
        if r.start != end || r.end < r.start {
            return None;
        }
        end = r.end;
    }

    Some(Range { start, end })
}

impl<'a> Strloin<'a> {
    /// Construct a new Strloin from the given string.
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Strloin { source }
    }

    /// Extracts a string from the given ranges; if the ranges form a single contiguous region,
    /// then the result will borrow from the source string. Otherwise, the ranges will be collected
    /// into an owned string.
    ///
    /// # Examples
    ///
    /// ```
    /// use strloin::Strloin;
    ///
    /// let strloin = Strloin::new("hello world");
    ///
    /// assert_eq!(strloin.from_ranges(&[0..5]), "hello"); // borrowed
    /// assert_eq!(strloin.from_ranges(&[0..5, 5..11]), "hello world"); // borrowed
    /// assert_eq!(strloin.from_ranges(&[0..5, 6..11]), "helloworld"); // owned
    /// ```
    #[must_use]
    pub fn from_ranges(&self, ranges: &[Range<usize>]) -> Cow<'a, str> {
        if let Some(range) = collapse_ranges(ranges) {
            return Borrowed(&self.source[range]);
        }

        Owned(
            ranges
                .iter()
                .map(|r| &self.source[r.clone()])
                .collect::<String>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapse_ranges() {
        use super::collapse_ranges;

        assert_eq!(collapse_ranges(&[]), Some(0..0));

        assert_eq!(collapse_ranges(&[0..0]), Some(0..0));
        assert_eq!(collapse_ranges(&[0..2]), Some(0..2));
        assert_eq!(collapse_ranges(&[2..0]), None);

        assert_eq!(collapse_ranges(&[0..2, 2..4]), Some(0..4));
        assert_eq!(collapse_ranges(&[3..2, 2..4]), None);
        assert_eq!(collapse_ranges(&[0..2, 4..6]), None);
        assert_eq!(collapse_ranges(&[0..2, 3..4]), None);
        assert_eq!(collapse_ranges(&[2..4, 0..2]), None);
        assert_eq!(collapse_ranges(&[0..2, 0..4]), None);
        assert_eq!(collapse_ranges(&[0..2, 2..1]), None);

        assert_eq!(collapse_ranges(&[0..2, 2..4, 4..6]), Some(0..6));
        assert_eq!(collapse_ranges(&[0..2, 3..5, 6..8]), None);
        assert_eq!(collapse_ranges(&[0..2, 3..5, 5..7]), None);
    }

    #[test]
    fn from_ranges() {
        macro_rules! from_ranges_ok {
            ($strloin:expr, $ranges:expr, $expected:expr, $is_borrow:expr) => {
                let got = $strloin.from_ranges($ranges);
                assert_eq!(got, $expected);

                // I don't see a good way to test beef externally
                #[cfg(not(feature = "beef"))]
                if $is_borrow {
                    assert!(matches!(got, Borrowed(_)), "expected borrow");
                } else {
                    assert!(matches!(got, Owned(_)), "expected owned");
                }
            };
        }

        let string = "hello world";
        let strloin = Strloin::new(&string);

        from_ranges_ok!(strloin, &[], "", true);
        from_ranges_ok!(strloin, &[0..5], "hello", true);
        from_ranges_ok!(strloin, &[6..11], "world", true);
        from_ranges_ok!(strloin, &[0..5, 5..11], "hello world", true);
        from_ranges_ok!(strloin, &[0..5, 6..11], "helloworld", false);
        from_ranges_ok!(strloin, &[6..11, 5..6, 0..5], "world hello", false);
        from_ranges_ok!(strloin, &[0..6, 0..5], "hello hello", false);
    }

    #[test]
    #[should_panic]
    fn invalid_range() {
        let string = "hello world";
        let strloin = Strloin::new(&string);
        let _ = strloin.from_ranges(&[1..0]);
    }

    #[test]
    #[should_panic]
    fn invalid_ranges() {
        let string = "hello world";
        let strloin = Strloin::new(&string);
        let _ = strloin.from_ranges(&[2..1, 1..4]);
    }
}
