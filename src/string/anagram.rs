pub fn check_anagram(s: &str, t: &str) -> bool {
    let mut s = s.to_ascii_lowercase().chars().collect::<Vec<_>>();
    let mut t = t.to_ascii_lowercase().chars().collect::<Vec<_>>();
    s.sort_unstable();
    t.sort_unstable();
    s == t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_anagram() {
        assert!(check_anagram("anagram", "nagaram"));
        assert!(!check_anagram("rat", "car"));
        assert!(check_anagram("abcde", "edcba"));
        assert!(check_anagram("sIlEnT", "LiStEn"));
    }
}
