//! Given two strings s1 and s2 consisting of lowercase letters.
//!
//! Determine whether the characters of one of the strings can be
//! rearranged to become another string.

pub fn check_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    s1.chars()
        .all(|c| s1.matches(c).count() == s2.matches(c).count())
}

#[cfg(test)]
mod tests {
    macro_rules! test_check_permutation {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                use super::check_permutation;
                let (s, t, expected) = $inputs;
                assert_eq!(check_permutation(s, t), expected);
                assert_eq!(check_permutation(t, s), expected);
                assert!(check_permutation(s, s));
                assert!(check_permutation(t, t));
            }
        )*
        }
    }

    test_check_permutation! {
        is_permutation: ("abc", "bca", true),
        not_permutation: ("abc", "bab", false),
        is_permutation_unicode: ("常威打来福", "来福打常威", true),
        not_permutation_unicode: ("常威打来福", "来福骂常威", false),
        empty: ("", "", true),
        different_length: ("abc", "abcd", false),
    }
}
