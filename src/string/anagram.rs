use std::collections::HashMap;

/// Custom error type representing an invalid character found in the input.
#[derive(Debug, PartialEq)]
pub enum AnagramError {
    NonAlphabeticCharacter,
}

/// Checks if two strings are anagrams, ignoring spaces and case sensitivity.
///
/// # Arguments
///
/// * `s` - First input string.
/// * `t` - Second input string.
///
/// # Returns
///
/// * `Ok(true)` if the strings are anagrams.
/// * `Ok(false)` if the strings are not anagrams.
/// * `Err(AnagramError)` if either string contains non-alphabetic characters.
pub fn check_anagram(s: &str, t: &str) -> Result<bool, AnagramError> {
    let s_cleaned = clean_string(s)?;
    let t_cleaned = clean_string(t)?;

    Ok(char_frequency(&s_cleaned) == char_frequency(&t_cleaned))
}

/// Cleans the input string by removing spaces and converting to lowercase.
/// Returns an error if any non-alphabetic character is found.
///
/// # Arguments
///
/// * `s` - Input string to clean.
///
/// # Returns
///
/// * `Ok(String)` containing the cleaned string (no spaces, lowercase).
/// * `Err(AnagramError)` if the string contains non-alphabetic characters.
fn clean_string(s: &str) -> Result<String, AnagramError> {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            if c.is_alphabetic() {
                Ok(c.to_ascii_lowercase())
            } else {
                Err(AnagramError::NonAlphabeticCharacter)
            }
        })
        .collect()
}

/// Computes the frequency of characters in a string.
///
/// # Arguments
///
/// * `s` - Input string.
///
/// # Returns
///
/// * A `HashMap` where the keys are characters and values are their frequencies.
fn char_frequency(s: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    freq
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (s, t, expected) = $test_case;
                    assert_eq!(check_anagram(s, t), expected);
                    assert_eq!(check_anagram(t, s), expected);
                }
            )*
        }
    }

    test_cases! {
        empty_strings: ("", "", Ok(true)),
        empty_and_non_empty: ("", "Ted Morgan", Ok(false)),
        single_char_same: ("z", "Z", Ok(true)),
        single_char_diff: ("g", "h", Ok(false)),
        valid_anagram_lowercase: ("cheater", "teacher", Ok(true)),
        valid_anagram_with_spaces: ("Madam Curie", "Radium came", Ok(true)),
        valid_anagram_mixed_cases: ("Satan", "Santa", Ok(true)),
        valid_anagram_awesome: ("Anna Madrigal", "A man and a girl", Ok(true)),
        non_anagram: ("rat", "car", Ok(false)),
        invalid_anagram_with_special_char: ("hello!", "world", Err(AnagramError::NonAlphabeticCharacter)),
        invalid_anagram_with_numeric_chars: ("test123", "321test", Err(AnagramError::NonAlphabeticCharacter)),
        invalid_anagram_with_symbols: ("check@anagram", "check@nagaram", Err(AnagramError::NonAlphabeticCharacter)),
        non_anagram_length_mismatch: ("abc", "abcd", Ok(false)),
    }
}
