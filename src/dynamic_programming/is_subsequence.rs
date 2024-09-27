//! A module for checking if one string is a subsequence of another string.
//!
//! A subsequence is formed by deleting some (can be none) of the characters
//! from the original string without disturbing the relative positions of the
//! remaining characters. This module provides a function to determine if
//! a given string is a subsequence of another string.

/// Checks if `sub` is a subsequence of `main`.
///
/// # Arguments
///
/// * `sub` - A string slice that may be a subsequence.
/// * `main` - A string slice that is checked against.
///
/// # Returns
///
/// Returns `true` if `sub` is a subsequence of `main`, otherwise returns `false`.
pub fn is_subsequence(sub: &str, main: &str) -> bool {
    let mut sub_iter = sub.chars().peekable();
    let mut main_iter = main.chars();

    while let Some(&sub_char) = sub_iter.peek() {
        match main_iter.next() {
            Some(main_char) if main_char == sub_char => {
                sub_iter.next();
            }
            None => return false,
            _ => {}
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! subsequence_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (sub, main, expected) = $test_case;
                    assert_eq!(is_subsequence(sub, main), expected);
                }
            )*
        };
    }

    subsequence_tests! {
        test_empty_subsequence: ("", "ahbgdc", true),
        test_empty_strings: ("", "", true),
        test_non_empty_sub_empty_main: ("abc", "", false),
        test_subsequence_found: ("abc", "ahbgdc", true),
        test_subsequence_not_found: ("axc", "ahbgdc", false),
        test_longer_sub: ("abcd", "abc", false),
        test_single_character_match: ("a", "ahbgdc", true),
        test_single_character_not_match: ("x", "ahbgdc", false),
        test_subsequence_at_start: ("abc", "abchello", true),
        test_subsequence_at_end: ("cde", "abcde", true),
        test_same_characters: ("aaa", "aaaaa", true),
        test_interspersed_subsequence: ("ace", "abcde", true),
        test_different_chars_in_subsequence: ("aceg", "abcdef", false),
        test_single_character_in_main_not_match: ("a", "b", false),
        test_single_character_in_main_match: ("b", "b", true),
        test_subsequence_with_special_chars: ("a1!c", "a1!bcd", true),
        test_case_sensitive: ("aBc", "abc", false),
        test_subsequence_with_whitespace: ("hello world", "h e l l o   w o r l d", true),
    }
}
