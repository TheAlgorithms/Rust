pub fn is_palindrome(s: &str) -> bool {
    let mut chars = s.chars();
    while let (Some(c1), Some(c2)) = (chars.next(), chars.next_back()) {
        if c1 != c2 {
            return false;
        }
    }
    true
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
