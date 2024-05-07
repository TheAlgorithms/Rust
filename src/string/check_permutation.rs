//! Given two strings s1 and s2.
//!
//! Determine whether the characters of one of the strings can be
//! rearranged to become another string.
use std::collections::HashMap;

pub fn check_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut map = HashMap::new();
    // Record the number of occurrences of the s1 character
    for c in s1.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    // Iterate through s2 , if encountered the number of occurrences of 0
    // indicates the existence of s1 does not exist in the character.
    for c in s2.chars() {
        let count = map.entry(c).or_insert(0);
        if *count == 0 {
            return false;
        } else {
            *count -= 1;
        }
    }

    true
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
        is_permutation1: ("aBc", "Bca", true),
        not_permutation: ("abc", "bab", false),
        is_permutation_unicode: ("常威打来福", "来福打常威", true),
        not_permutation_unicode: ("常威打来福", "来福骂常威", false),
        empty: ("", "", true),
        different_length: ("abc", "abcd", false),
    }
}
