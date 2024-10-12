//! This module provides functionality to check if a given string is an isogram.
//! An isogram is a word or phrase in which no letter occurs more than once.

use std::collections::HashMap;

/// Enum representing possible errors that can occur while checking for isograms.
#[derive(Debug, PartialEq, Eq)]
pub enum IsogramError {
    /// Indicates that the input contains a non-alphabetic character.
    NonAlphabeticCharacter,
}

/// Counts the occurrences of each alphabetic character in a given string.
///
/// This function takes a string slice as input. It counts how many times each alphabetic character
/// appears in the input string and returns a hashmap where the keys are characters and the values
/// are their respective counts.
///
/// # Arguments
///
/// * `s` - A string slice that contains the input to count characters from.
///
/// # Errors
///
/// Returns an error if the input contains non-alphabetic characters (excluding spaces).
///
/// # Note
///
/// This function treats uppercase and lowercase letters as equivalent (case-insensitive).
/// Spaces are ignored and do not affect the character count.
fn count_letters(s: &str) -> Result<HashMap<char, usize>, IsogramError> {
    let mut letter_counts = HashMap::new();

    for ch in s.to_ascii_lowercase().chars() {
        if !ch.is_ascii_alphabetic() && ch != ' ' {
            return Err(IsogramError::NonAlphabeticCharacter);
        }

        if ch.is_ascii_alphabetic() {
            *letter_counts.entry(ch).or_insert(0) += 1;
        }
    }

    Ok(letter_counts)
}

/// Checks if the given input string is an isogram.
///
/// This function takes a string slice as input. It counts the occurrences of each
/// alphabetic character (ignoring case and spaces).
///
/// # Arguments
///
/// * `input` - A string slice that contains the input to check for isogram properties.
///
/// # Return
///
/// - `Ok(true)` if all characters appear only once, or `Ok(false)` if any character appears more than once.
/// - `Err(IsogramError::NonAlphabeticCharacter) if the input contains any non-alphabetic characters.
pub fn is_isogram(s: &str) -> Result<bool, IsogramError> {
    let letter_counts = count_letters(s)?;
    Ok(letter_counts.values().all(|&count| count == 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! isogram_tests {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $tc;
                    assert_eq!(is_isogram(input), expected);
                }
            )*
        };
    }

    isogram_tests! {
        isogram_simple: ("isogram", Ok(true)),
        isogram_case_insensitive: ("Isogram", Ok(true)),
        isogram_with_spaces: ("a b c d e", Ok(true)),
        isogram_mixed: ("Dermatoglyphics", Ok(true)),
        isogram_long: ("Subdermatoglyphic", Ok(true)),
        isogram_german_city: ("Malitzschkendorf", Ok(true)),
        perfect_pangram: ("Cwm fjord bank glyphs vext quiz", Ok(true)),
        isogram_sentences: ("The big dwarf only jumps", Ok(true)),
        isogram_french: ("Lampez un fort whisky", Ok(true)),
        isogram_portuguese: ("Velho traduz sim", Ok(true)),
        isogram_spanis: ("Centrifugadlos", Ok(true)),
        invalid_isogram_with_repeated_char: ("hello", Ok(false)),
        invalid_isogram_with_numbers: ("abc123", Err(IsogramError::NonAlphabeticCharacter)),
        invalid_isogram_with_special_char: ("abc!", Err(IsogramError::NonAlphabeticCharacter)),
        invalid_isogram_with_comma: ("Velho, traduz sim", Err(IsogramError::NonAlphabeticCharacter)),
        invalid_isogram_with_spaces: ("a b c d a", Ok(false)),
        invalid_isogram_with_repeated_phrase: ("abcabc", Ok(false)),
        isogram_empty_string: ("", Ok(true)),
        isogram_single_character: ("a", Ok(true)),
        invalid_isogram_multiple_same_characters: ("aaaa", Ok(false)),
        invalid_isogram_with_symbols: ("abc@#$%", Err(IsogramError::NonAlphabeticCharacter)),
    }
}
