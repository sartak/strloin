use std::ops::Range;

#[must_use]
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
            ($input:expr, $expected:expr) => {
                assert_eq!(collapse_ranges($input), $expected);
            };
        }

        ranges_ok!(&[], Some(0..0));

        ranges_ok!(&[0..0], Some(0..0));
        ranges_ok!(&[0..2], Some(0..2));
        ranges_ok!(&[2..0], None);

        ranges_ok!(&[0..2, 2..4], Some(0..4));
        ranges_ok!(&[3..2, 2..4], None);
        ranges_ok!(&[0..2, 4..6], None);
        ranges_ok!(&[0..2, 3..4], None);
        ranges_ok!(&[2..4, 0..2], None);
        ranges_ok!(&[0..2, 0..4], None);
        ranges_ok!(&[0..2, 2..1], None);

        ranges_ok!(&[0..2, 2..4, 4..6], Some(0..6));
        ranges_ok!(&[0..2, 3..5, 6..8], None);
        ranges_ok!(&[0..2, 3..5, 5..7], None);
    }
}
