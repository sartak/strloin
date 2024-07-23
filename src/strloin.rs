use crate::cow::{Borrowed, Cow, Owned};
use crate::ranges::{collapse_ranges, Ranges};
use std::ops::Range;

/// Holds a source string for conditionally borrowing.
#[derive(Debug, Clone)]
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

    /// Extracts a string from the given [`Ranges`] object; if the ranges form a single contiguous
    /// region, then the result will borrow from the source string. Otherwise, the ranges will be
    /// collected into an owned string. If you're incrementally building up the list of ranges and
    /// checking each time, using `from_ranges_obj` is more efficient.
    ///
    /// # Examples
    ///
    /// ```
    /// use strloin::{Strloin, Ranges};
    ///
    /// let strloin = Strloin::new("hello world");
    ///
    /// let mut ranges = Ranges::from(0..5);
    ///
    /// assert_eq!(strloin.from_ranges_obj(&ranges), "hello"); // borrowed
    ///
    /// ranges.push(5..11);
    /// assert_eq!(strloin.from_ranges_obj(&ranges), "hello world"); // borrowed
    ///
    /// ranges.push(5..11);
    /// assert_eq!(strloin.from_ranges_obj(&ranges), "hello world world"); // owned
    /// ```
    #[must_use]
    pub fn from_ranges_obj(&self, ranges: &Ranges) -> Cow<'a, str> {
        match ranges.ranges.as_slice() {
            &[] => Borrowed(""),
            [range] => Borrowed(&self.source[range.clone()]),
            ranges => Owned(
                ranges
                    .iter()
                    .map(|r| &self.source[r.clone()])
                    .collect::<String>(),
            ),
        }
    }
}

impl<'a> From<&'a str> for Strloin<'a> {
    fn from(source: &'a str) -> Self {
        Strloin::new(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_ranges() {
        macro_rules! from_ranges_ok {
            ($strloin:expr, $input:expr, $expected:expr, $is_borrow:expr) => {
                let strloin = &$strloin;
                let input = $input;
                let expected = $expected;

                let got_from_slice = strloin.from_ranges(input);
                assert_eq!(got_from_slice, expected, "from_ranges");

                let mut ranges = Ranges::new();
                for range in input {
                    ranges.push(range.clone());
                }
                let got_from_obj = strloin.from_ranges_obj(&ranges);
                assert_eq!(got_from_obj, expected, "from_ranges_obj");

                if $is_borrow {
                    assert!(
                        matches!(got_from_slice, Borrowed(_)),
                        "expected borrow from ranges slice"
                    );
                    assert!(
                        matches!(got_from_obj, Borrowed(_)),
                        "expected borrow from ranges obj"
                    );
                } else {
                    assert!(
                        matches!(got_from_slice, Owned(_)),
                        "expected owned from ranges slice"
                    );
                    assert!(
                        matches!(got_from_obj, Owned(_)),
                        "expected owned from ranges obj"
                    );
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
