use crate::splitter::utils::sliding_windows;

/// Splits a slice into sub-slices of length `n`, with step and optional tail, borrowing from the input.
///
/// - `input`: The input slice.
/// - `n`: Number of elements per window.
/// - `step`: Step between window starts (`0` means `step = n`).
/// - `keep_tail`: If true, includes a final sub-slice for any remaining elements at the end.
///
/// Returns a vector of sub-slices (`&[T]`) borrowing from the input slice.
pub fn slice_by_windows_borrowed<T>(
    input: &[T],
    n: usize,
    step: usize,
    keep_tail: bool,
) -> Vec<&[T]> {
    let len = input.len();

    let mut out = Vec::new();
    for r in sliding_windows(len, n, step, keep_tail) {
        out.push(&input[r]);
    }
    out
}


/// A splitter for dividing a list (slice) into sub-slices of specified length by element count.
///
/// - `input`: The input list (slice) to split.
/// - `n`: The number of elements per sub-slice.
/// - `step`: Step between window starts (`0` means no overlap).
/// - `keep_tail`: If true, includes a final sub-slice for any remaining elements at the end.
///
/// Provides methods to get borrowed sub-slices (`split`) or owned sub-vectors (`out`).
pub struct SliceSplitter<'a, T> {
    input: &'a [T],
    n: usize,
    step: usize,
    keep_tail: bool,
}

impl<'a, T> SliceSplitter<'a, T> {
    /// Create a new SliceSplitter.
    pub fn new(input: &'a [T], n: usize, step: usize, keep_tail: bool) -> Self {
        Self { input, n, step, keep_tail }
    }

    /// Return a vector of borrowed sub-slices.
    pub fn split(&self) -> Vec<&'a [T]> {
        slice_by_windows_borrowed(self.input, self.n, self.step, self.keep_tail)
    }

    /// Return a vector of owned sub-vectors.
    pub fn out(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        self.split().into_iter().map(|s| s.to_vec()).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_by_windows_borrowed() {
        let data = [1, 2, 3, 4, 5, 6, 7];
        let windows = slice_by_windows_borrowed(&data, 3, 0, true);
        assert_eq!(windows, vec![&[1,2,3][..], &[4,5,6][..], &[7][..]]);
    }
    #[test]
    fn test_slice_by_windows_borrowed_strings() {
        let data = ["AA", "BB", "CC", "DD", "EE", "FF", "GG"];
        let windows = slice_by_windows_borrowed(&data, 3, 0, true);
        assert_eq!(
            windows,
            vec![
                &["AA", "BB", "CC"][..],
                &["DD", "EE", "FF"][..],
                &["GG"][..]
            ]
        );
    }
}