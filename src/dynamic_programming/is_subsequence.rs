// Given two strings str1 and str2, return true if str1 is a subsequence of str2, or false otherwise.
// A subsequence of a string is a new string that is formed from the original string
// by deleting some (can be none) of the characters without disturbing the relative
// positions of the remaining characters.
// (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
pub fn is_subsequence(str1: &str, str2: &str) -> bool {
    let mut it1 = 0;
    let mut it2 = 0;

    let byte1 = str1.as_bytes();
    let byte2 = str2.as_bytes();

    while it1 < str1.len() && it2 < str2.len() {
        if byte1[it1] == byte2[it2] {
            it1 += 1;
        }

        it2 += 1;
    }

    it1 == str1.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(is_subsequence("abc", "ahbgdc"));
        assert!(!is_subsequence("axc", "ahbgdc"));
    }
}
