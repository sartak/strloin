#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::{borrow::Cow, ops::Range};

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

    for r in rs {
        if r.start != end {
            return None;
        }
        end = r.end;
    }

    Some(Range { start, end })
}

impl<'a> Strloin<'a> {
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Strloin { source }
    }

    #[must_use]
    pub fn from_ranges(&self, ranges: &[Range<usize>]) -> Cow<'a, str> {
        if let Some(range) = collapse_ranges(ranges) {
            return Cow::Borrowed(&self.source[range]);
        }

        Cow::Owned(
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

        assert_eq!(collapse_ranges(&[0..2]), Some(0..2));

        assert_eq!(collapse_ranges(&[0..2, 2..4]), Some(0..4));
        assert_eq!(collapse_ranges(&[0..2, 4..6]), None);
        assert_eq!(collapse_ranges(&[0..2, 3..4]), None);
        assert_eq!(collapse_ranges(&[2..4, 0..2]), None);

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
                if $is_borrow {
                    assert!(matches!(got, Cow::Borrowed(_)), "expected borrow");
                } else {
                    assert!(matches!(got, Cow::Owned(_)), "expected owned");
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
}
