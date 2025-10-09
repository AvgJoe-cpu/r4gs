/// Splits a string into a vector of words separated by whitespace.
pub fn dummy(string: &str) -> Vec<String> {
    let mut result = Vec::new();

    for word in string.split_whitespace() {
        result.push(word.to_string());
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_basic() {
        let input = "hello world from rust";
        let expected = vec![
            "hello".to_string(),
            "world".to_string(),
            "from".to_string(),
            "rust".to_string(),
        ];

        let result = dummy(input);
        assert_eq!(result, expected);
    }
}
