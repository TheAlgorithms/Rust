use std::collections::HashSet;

/// Represents possible errors that can occur when checking for lipograms.
#[derive(Debug, PartialEq, Eq)]
pub enum LipogramError {
    /// Indicates that a non-alphabetic character was found in the input.
    NonAlphabeticCharacter,
    /// Indicates that a missing character is not in lowercase.
    NonLowercaseMissingChar,
}

/// Computes the set of missing alphabetic characters from the input string.
///
/// # Arguments
///
/// * `in_str` - A string slice that contains the input text.
///
/// # Returns
///
/// Returns a `HashSet<char>` containing the lowercase alphabetic characters that are not present in `in_str`.
fn compute_missing(in_str: &str) -> HashSet<char> {
    let alphabet: HashSet<char> = ('a'..='z').collect();

    let letters_used: HashSet<char> = in_str
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    alphabet.difference(&letters_used).cloned().collect()
}

/// Checks if the provided string is a lipogram, meaning it is missing specific characters.
///
/// # Arguments
///
/// * `lipogram_str` - A string slice that contains the text to be checked for being a lipogram.
/// * `missing_chars` - A reference to a `HashSet<char>` containing the expected missing characters.
///
/// # Returns
///
/// Returns `Ok(true)` if the string is a lipogram that matches the provided missing characters,
/// `Ok(false)` if it does not match, or a `LipogramError` if the input contains invalid characters.
pub fn is_lipogram(
    lipogram_str: &str,
    missing_chars: &HashSet<char>,
) -> Result<bool, LipogramError> {
    for &c in missing_chars {
        if !c.is_lowercase() {
            return Err(LipogramError::NonLowercaseMissingChar);
        }
    }

    for c in lipogram_str.chars() {
        if !c.is_ascii_alphabetic() && !c.is_whitespace() {
            return Err(LipogramError::NonAlphabeticCharacter);
        }
    }

    let missing = compute_missing(lipogram_str);
    Ok(missing == *missing_chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_lipogram {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, missing_chars, expected) = $tc;
                    assert_eq!(is_lipogram(input, &missing_chars), expected);
                }
            )*
        }
    }

    test_lipogram! {
        perfect_pangram: (
            "The quick brown fox jumps over the lazy dog",
            HashSet::from([]),
            Ok(true)
        ),
        lipogram_single_missing: (
            "The quick brown fox jumped over the lazy dog",
            HashSet::from(['s']),
            Ok(true)
        ),
        lipogram_multiple_missing: (
            "The brown fox jumped over the lazy dog",
            HashSet::from(['q', 'i', 'c', 'k', 's']),
            Ok(true)
        ),
        long_lipogram_single_missing: (
            "A jovial swain should not complain of any buxom fair who mocks his pain and thinks it gain to quiz his awkward air",
            HashSet::from(['e']),
            Ok(true)
        ),
        invalid_non_lowercase_chars: (
            "The quick brown fox jumped over the lazy dog",
            HashSet::from(['X']),
            Err(LipogramError::NonLowercaseMissingChar)
        ),
        invalid_non_alphabetic_input: (
            "The quick brown fox jumps over the lazy dog 123@!",
            HashSet::from([]),
            Err(LipogramError::NonAlphabeticCharacter)
        ),
    }
}
