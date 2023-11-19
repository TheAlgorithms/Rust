pub fn check_anagram(s: &str, t: &str) -> bool {
    sort_string(s) == sort_string(t)
}

fn sort_string(s: &str) -> Vec<char> {
    let mut res: Vec<char> = s.to_ascii_lowercase().chars().collect::<Vec<_>>();
    res.sort_unstable();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_anagram() {
        assert!(check_anagram("", ""));
        assert!(check_anagram("A", "a"));
        assert!(check_anagram("anagram", "nagaram"));
        assert!(check_anagram("abcde", "edcba"));
        assert!(check_anagram("sIlEnT", "LiStEn"));

        assert!(!check_anagram("", "z"));
        assert!(!check_anagram("a", "z"));
        assert!(!check_anagram("rat", "car"));
    }
}
