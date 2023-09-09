use std::ops::Range;

/// A data structure for incrementally building a list of ranges.
#[derive(Debug, Default, Clone)]
pub struct Ranges {
    pub ranges: Vec<Range<usize>>,
}

impl Ranges {
    /// Construct a new empty [`Ranges`].
    #[must_use]
    pub const fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    /// Construct a new empty [`Ranges`] with the given capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ranges: Vec::with_capacity(capacity),
        }
    }

    /// Construct a new [`Ranges`] from a single range.
    #[must_use]
    pub fn from_range(range: Range<usize>) -> Self {
        Self {
            ranges: vec![range],
        }
    }

    /// Construct a new [`Ranges`] from a single range with the given capacity.
    #[must_use]
    pub fn from_range_with_capacity(range: Range<usize>, capacity: usize) -> Self {
        let mut ranges = Vec::with_capacity(capacity);
        ranges.push(range);
        Self { ranges }
    }

    /// Adds a new range to the [`Ranges`], collapsing if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use strloin::Ranges;
    ///
    /// let mut ranges = Ranges::from_range(0..5);
    /// ranges.push(5..11);
    /// assert_eq!(ranges.ranges, vec![0..11]);
    ///
    /// ranges.push(4..8);
    /// assert_eq!(ranges.ranges, vec![0..11, 4..8]);
    /// ```
    pub fn push(&mut self, range: Range<usize>) {
        if let Some(last) = self.ranges.last_mut() {
            #[allow(clippy::suspicious_operation_groupings)]
            if range.start == last.end && last.start < last.end && range.start < range.end {
                last.end = range.end;
                return;
            }
        }

        self.ranges.push(range);
    }

    /// Adds a new range to the [`Ranges`], collapsing if possible.
    ///
    /// # Safety
    ///
    /// `push_unchecked` does not check whether the new range has `start` <= `end`. The caller must guarantee that the provided range is valid, otherwise it may
    /// produce bogus results. (Note that it is perfectly acceptable to push a
    /// range overlaps or precedes what's already in the `Ranges`)
    pub unsafe fn push_unchecked(&mut self, range: Range<usize>) {
        if let Some(last) = self.ranges.last_mut() {
            if range.start == last.end {
                last.end = range.end;
                return;
            }
        }

        self.ranges.push(range);
    }

    /// Removes all elements from the [`Ranges`].
    pub fn clear(&mut self) {
        self.ranges.clear();
    }
}

impl From<Range<usize>> for Ranges {
    fn from(range: Range<usize>) -> Self {
        Self::from_range(range)
    }
}

impl FromIterator<Range<usize>> for Ranges {
    fn from_iter<I: IntoIterator<Item = Range<usize>>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (capacity, _) = iter.size_hint();
        let mut ranges = Self::with_capacity(capacity);

        for range in iter {
            ranges.push(range);
        }

        ranges
    }
}

impl Extend<Range<usize>> for Ranges {
    fn extend<T: IntoIterator<Item = Range<usize>>>(&mut self, iter: T) {
        for range in iter {
            self.push(range);
        }
    }
}

/// Collapse a slice of ranges into a single contiguous range, if possible.
///
/// # Examples
///
/// ```
/// use strloin::collapse_ranges;
///
/// assert_eq!(collapse_ranges(&[0..5]), Some(0..5));
/// assert_eq!(collapse_ranges(&[0..5, 5..11]), Some(0..11));
/// assert_eq!(collapse_ranges(&[0..5, 6..11]), None);
/// ```
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn collapse_ranges(ranges: &[Range<usize>]) -> Option<Range<usize>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges() {
        macro_rules! ranges_ok {
            ($input:expr, $expected_collapse:expr, $expected_ranges:expr) => {
                let input = $input;

                assert_eq!(
                    collapse_ranges(input),
                    $expected_collapse,
                    "collapse_ranges"
                );

                let mut ranges = Ranges::new();
                for range in input {
                    ranges.push(range.clone());
                }

                assert_eq!(ranges.ranges, $expected_ranges, "Ranges");
            };
        }

        ranges_ok!(&[], Some(0..0), &[]);

        ranges_ok!(&[0..0], Some(0..0), &[0..0]);
        ranges_ok!(&[0..2], Some(0..2), &[0..2]);
        ranges_ok!(&[2..0], None, &[2..0]);

        ranges_ok!(&[0..2, 2..4], Some(0..4), &[0..4]);
        ranges_ok!(&[3..2, 2..4], None, &[3..2, 2..4]);
        ranges_ok!(&[0..2, 4..6], None, &[0..2, 4..6]);
        ranges_ok!(&[0..2, 3..4], None, &[0..2, 3..4]);
        ranges_ok!(&[2..4, 0..2], None, &[2..4, 0..2]);
        ranges_ok!(&[0..2, 0..4], None, &[0..2, 0..4]);
        ranges_ok!(&[0..2, 2..1], None, &[0..2, 2..1]);

        ranges_ok!(&[0..2, 2..4, 4..6], Some(0..6), &[0..6]);
        ranges_ok!(&[0..2, 3..5, 6..8], None, &[0..2, 3..5, 6..8]);
        ranges_ok!(&[0..2, 3..5, 5..7], None, &[0..2, 3..7]);
    }
}
