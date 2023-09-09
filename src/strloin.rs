use crate::cow::{Borrowed, Cow, Owned};
use crate::ranges::collapse_ranges;
use std::ops::Range;

/// Holds a source string for conditionally borrowing.
pub struct Strloin<'a> {
    pub source: &'a str,
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
