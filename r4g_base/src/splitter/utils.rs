use std::ops::Range;

/// Returns an iterator over fixed-size, possibly overlapping ranges within a slice of length `len`.
///
/// - `n`: Size of each range.
/// - `step`: Step between the starts of ranges (`0` means `step = n`, i.e., non-overlapping).
/// - `keep_tail`: If true, includes a final range for any remaining elements at the end.
///
/// Each item produced is a `Range<usize>` representing the bounds of the subrange.
#[inline]
pub fn sliding_windows(
    len: usize,
    n: usize,
    step: usize, // 0 ⇒ hop = n
    keep_tail: bool,
) -> impl Iterator<Item = Range<usize>> {

    let hop = if step == 0 { n } else { step };
    let mut i = 0usize;
    let mut emitted = false;

    std::iter::from_fn(move || {
        if i + n <= len {
            let r = i..i + n;
            i = i.saturating_add(hop);
            return Some(r);
        }
        if keep_tail && !emitted && i < len {
            emitted = true;
            return Some(i..len);
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_non_overlapping() {
        let ranges: Vec<_> = sliding_windows(10, 3, 0, false).collect();
        assert_eq!(ranges, vec![0..3, 3..6, 6..9]);
    }

    #[test]
    fn overlapping_ranges() {
        let ranges: Vec<_> = sliding_windows(8, 5, 2, true).collect();
        assert_eq!(ranges, vec![0..5, 2..7, 4..8]);
    }

    #[test]
    fn with_tail() {
        let ranges: Vec<_> = sliding_windows(10, 4, 0, true).collect();
        assert_eq!(ranges, vec![0..4, 4..8, 8..10]);
    }

    #[test]
    fn n_greater_than_len_with_tail() {
        let ranges: Vec<_> = sliding_windows(5, 10, 0, true).collect();
        assert_eq!(ranges, vec![0..5]);
    }

    #[test]
    fn n_greater_than_len_without_tail() {
        let ranges: Vec<_> = sliding_windows(5, 10, 0, false).collect();
        assert!(ranges.is_empty());
    }

    #[test]
    fn empty_input() {
        let ranges: Vec<_> = sliding_windows(0, 3, 0, false).collect();
        assert!(ranges.is_empty());
    }

    #[test]
    fn zero_width_ranges() {
        let ranges: Vec<_> = sliding_windows(3, 0, 1, false).collect();
        assert_eq!(ranges, vec![0..0, 1..1, 2..2, 3..3]);
    }
}