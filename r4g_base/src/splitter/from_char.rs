use crate::splitter::utils::sliding_windows;

/// Splits a UTF-8 string into borrowed substrings of `n` characters each, optionally overlapping, with optional tail.
///
/// - `input`: The input string.
/// - `n`: The number of characters per substring.
/// - `step`: Step between window starts (`0` means `step = n`).
/// - `keep_tail`: If true, includes a final substring for remaining characters at the end.
///
/// Returns a vector of string slices (`&str`) borrowing from the input.
pub fn utf8_by_chars_borrowed(
    input: &str,
    n: usize,
    step: usize,
    keep_tail: bool,
) -> Vec<&str> {
    let char_indices: Vec<usize> = input.char_indices().map(|(i, _)| i).collect();
    let num_chars = char_indices.len();

    let mut out = Vec::new();
    for r in sliding_windows(num_chars, n, step, keep_tail) {
        let byte_start = char_indices[r.start];
        let byte_end = if r.end < num_chars {
            char_indices[r.end]
        } else {
            input.len()
        };
        out.push(&input[byte_start..byte_end]);
    }
    out
}

/// Converts a vector of string slices into a vector of owned strings.
pub fn bulk_to_owned_into(windows: Vec<&str>) -> Vec<String> {
    let mut out = Vec::with_capacity(windows.len());
    for s in windows {
        out.push(s.to_string());
    }
    out
}


/// A splitter for dividing a UTF-8 string into substrings of specified length by character count.
///
/// - `input`: The input string slice to split.
/// - `n`: The number of characters per substring.
/// - `step`: Step between window starts (`0` means no overlap).
/// - `keep_tail`: If true, includes a final substring for any remaining characters at the end.
///
/// Provides methods to get borrowed string slices (`split`) or owned `String`s (`out`).
pub struct Utf8Splitter<'a> {
    input: &'a str,
    n: usize,
    step: usize,
    keep_tail: bool,
}

impl<'a> Utf8Splitter<'a> {
    pub fn new(input: &'a str, n: usize, step: usize, keep_tail: bool) -> Self {
        Self { input, n, step, keep_tail }
    }

    pub fn split(&self) -> Vec<&'a str> {
        utf8_by_chars_borrowed(self.input, self.n, self.step, self.keep_tail)
    }

    pub fn out(&self) -> Vec<String> {
        bulk_to_owned_into(self.split())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_ascii_nonoverlapping() {
        let s = "abcdef";
        let v = utf8_by_chars_borrowed(s, 2, 0, false);
        assert_eq!(v, vec!["ab", "cd", "ef"]);
    }

    #[test]
    fn overlapping_ascii() {
        let s = "abcdef";
        let v = utf8_by_chars_borrowed(s, 3, 1, false);
        assert_eq!(v, vec!["abc", "bcd", "cde", "def"]);
    }

    #[test]
    fn multi_byte_unicode() {
        let s = "a😀b😃c";
        let v = utf8_by_chars_borrowed(s, 2, 0, false);
        // Should be ["a😀", "b😃"]
        assert_eq!(v, vec!["a😀", "b😃"]);
    }

    #[test]
    fn with_tail() {
        let s = "abcdefg";
        let v = utf8_by_chars_borrowed(s, 3, 0, true);
        assert_eq!(v, vec!["abc", "def", "g"]);
    }
}