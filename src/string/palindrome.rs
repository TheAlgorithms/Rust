pub fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindromes() {
        assert!(is_palindrome("abcba"));
        assert!(is_palindrome("abba"));
        assert!(is_palindrome("a"));
        assert!(is_palindrome("arcra"));
        assert!(!is_palindrome("abcde"));
        assert!(!is_palindrome("aaaabbbb"));
    }
}
