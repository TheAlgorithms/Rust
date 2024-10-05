//! This module provides functionality to check if a given string is a pangram.
//!
//! A pangram is a sentence that contains every letter of the alphabet at least once.
//! This module can distinguish between a non-pangram, a regular pangram, and a
//! perfect pangram, where each letter appears exactly once.

use std::collections::HashSet;

/// Represents the status of a string in relation to the pangram classification.
#[derive(PartialEq, Debug)]
pub enum PangramStatus {
    NotPangram,
    Pangram,
    PerfectPangram,
}

/// Determines if the input string is a pangram, and classifies it as either a regular or perfect pangram.
///
/// # Arguments
///
/// * `pangram_str` - A reference to the string slice to be checked for pangram status.
///
/// # Returns
///
/// A `PangramStatus` enum indicating whether the string is a pangram, and if so, whether it is a perfect pangram.
pub fn is_pangram(pangram_str: &str) -> PangramStatus {
    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut letter_counts = std::collections::HashMap::new();

    for ch in pangram_str
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
    {
        *letter_counts.entry(ch).or_insert(0) += 1;
    }

    let unique_letters: HashSet<_> = letter_counts.keys().cloned().collect();

    if unique_letters != alphabet {
        return PangramStatus::NotPangram;
    }

    if letter_counts.values().all(|&count| count == 1) {
        PangramStatus::PerfectPangram
    } else {
        PangramStatus::Pangram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! pangram_tests {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $tc;
                    assert_eq!(is_pangram(input), expected);
                }
            )*
        };
    }

    pangram_tests! {
        test_not_pangram_simple: ("This is not a pangram", PangramStatus::NotPangram),
        test_not_pangram_day: ("today is a good day", PangramStatus::NotPangram),
        test_not_pangram_almost: ("this is almost a pangram but it does not have bcfghjkqwxy and the last letter", PangramStatus::NotPangram),
        test_pangram_standard: ("The quick brown fox jumps over the lazy dog", PangramStatus::Pangram),
        test_pangram_boxer: ("A mad boxer shot a quick, gloved jab to the jaw of his dizzy opponent", PangramStatus::Pangram),
        test_pangram_discotheques: ("Amazingly few discotheques provide jukeboxes", PangramStatus::Pangram),
        test_pangram_zebras: ("How vexingly quick daft zebras jump", PangramStatus::Pangram),
        test_perfect_pangram_jock: ("Mr. Jock, TV quiz PhD, bags few lynx", PangramStatus::PerfectPangram),
        test_empty_string: ("", PangramStatus::NotPangram),
        test_repeated_letter: ("aaaaa", PangramStatus::NotPangram),
        test_non_alphabetic: ("12345!@#$%", PangramStatus::NotPangram),
        test_mixed_case_pangram: ("ThE QuiCk BroWn FoX JumPs OveR tHe LaZy DoG", PangramStatus::Pangram),
        test_perfect_pangram_with_symbols: ("Mr. Jock, TV quiz PhD, bags few lynx!", PangramStatus::PerfectPangram),
        test_long_non_pangram: (&"a".repeat(1000), PangramStatus::NotPangram),
        test_near_pangram_missing_one_letter: ("The quick brown fox jumps over the lazy do", PangramStatus::NotPangram),
        test_near_pangram_missing_two_letters: ("The quick brwn f jumps ver the lazy dg", PangramStatus::NotPangram),
        test_near_pangram_with_special_characters: ("Th3 qu!ck brown f0x jumps 0v3r th3 l@zy d0g.", PangramStatus::NotPangram),
    }
}
