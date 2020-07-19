pub fn longest_common_subsequence(s1: &str, s2: &str, s1_len: &usize, s2_len: &usize) -> usize {
    if *s1_len == 0 || *s2_len == 0 {
        0
    } else if s1.chars().nth(s1_len - 1).unwrap() == s2.chars().nth(s2_len - 1).unwrap() {
        1 + longest_common_subsequence(s1, s2, &(*s1_len - 1), &(*s2_len - 1))
    } else {
        std::cmp::max(
            longest_common_subsequence(s1, s2, &s1_len, &(*s2_len - 1)),
            longest_common_subsequence(s1, s2, &(*s1_len - 1), &s2_len),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let a = "AGGTAB";
        let b = "GXTXAYB";
        let lcs = longest_common_subsequence(&a, &b, &a.len(), &b.len());
        assert_eq!(lcs, 4);
    }

    #[test]
    fn empty() {
        let a = "";
        let b = "";
        let lcs = longest_common_subsequence(&a, &b, &a.len(), &b.len());
        assert_eq!(lcs, 0);
    }

    #[test]
    fn one_empty() {
        let a = "AGGTAB";
        let b = "";
        let lcs = longest_common_subsequence(&a, &b, &a.len(), &b.len());
        assert_eq!(lcs, 0);
    }
}
