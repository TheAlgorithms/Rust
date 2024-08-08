//! Knuth-Morris-Pratt string matching algorithm implementation in Rust.
//!
//! This module contains the implementation of the KMP algorithm, which is used for finding
//! occurrences of a pattern string within a text string efficiently. The algorithm preprocesses
//! the pattern to create a partial match table, which allows for efficient searching.

/// Finds all occurrences of the pattern in the given string using the Knuth-Morris-Pratt algorithm.
///
/// # Arguments
///
/// * `string` - The string to search within.
/// * `pattern` - The pattern string to search for.
///
/// # Returns
///
/// A vector of starting indices where the pattern is found in the string. If the pattern or the
/// string is empty, an empty vector is returned.
pub fn knuth_morris_pratt(string: &str, pattern: &str) -> Vec<usize> {
    if string.is_empty() || pattern.is_empty() {
        return vec![];
    }

    let text_chars = string.chars().collect::<Vec<char>>();
    let pattern_chars = pattern.chars().collect::<Vec<char>>();
    let partial_match_table = build_partial_match_table(&pattern_chars);
    find_pattern(&text_chars, &pattern_chars, &partial_match_table)
}

/// Builds the partial match table (also known as "prefix table") for the given pattern.
///
/// The partial match table is used to skip characters while matching the pattern in the text.
/// Each entry at index `i` in the table indicates the length of the longest proper prefix of
/// the substring `pattern[0..i]` which is also a suffix of this substring.
///
/// # Arguments
///
/// * `pattern_chars` - The pattern string as a slice of characters.
///
/// # Returns
///
/// A vector representing the partial match table.
fn build_partial_match_table(pattern_chars: &[char]) -> Vec<usize> {
    let mut partial_match_table = vec![0];
    pattern_chars
        .iter()
        .enumerate()
        .skip(1)
        .for_each(|(index, &char)| {
            let mut length = partial_match_table[index - 1];
            while length > 0 && pattern_chars[length] != char {
                length = partial_match_table[length - 1];
            }
            partial_match_table.push(if pattern_chars[length] == char {
                length + 1
            } else {
                length
            });
        });
    partial_match_table
}

/// Finds all occurrences of the pattern in the given string using the precomputed partial match table.
///
/// This function iterates through the string and uses the partial match table to efficiently find
/// all starting indices of the pattern in the string.
///
/// # Arguments
///
/// * `text_chars` - The string to search within as a slice of characters.
/// * `pattern_chars` - The pattern string to search for as a slice of characters.
/// * `partial_match_table` - The precomputed partial match table for the pattern.
///
/// # Returns
///
/// A vector of starting indices where the pattern is found in the string.
fn find_pattern(
    text_chars: &[char],
    pattern_chars: &[char],
    partial_match_table: &[usize],
) -> Vec<usize> {
    let mut result_indices = vec![];
    let mut match_length = 0;

    text_chars
        .iter()
        .enumerate()
        .for_each(|(text_index, &text_char)| {
            while match_length > 0 && text_char != pattern_chars[match_length] {
                match_length = partial_match_table[match_length - 1];
            }
            if text_char == pattern_chars[match_length] {
                match_length += 1;
            }
            if match_length == pattern_chars.len() {
                result_indices.push(text_index + 1 - match_length);
                match_length = partial_match_table[match_length - 1];
            }
        });

    result_indices
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_knuth_morris_pratt {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, pattern, expected) = $inputs;
                    assert_eq!(knuth_morris_pratt(input, pattern), expected);
                }
            )*
        }
    }

    test_knuth_morris_pratt! {
        each_letter_matches: ("aaa", "a", vec![0, 1, 2]),
        a_few_seperate_matches: ("abababa", "ab", vec![0, 2, 4]),
        unicode: ("അഅഅ", "അ", vec![0, 1, 2]),
        unicode_no_match_but_similar_bytes: (
            &String::from_utf8(vec![224, 180, 133]).unwrap(),
            &String::from_utf8(vec![224, 180, 132]).unwrap(),
            vec![]
        ),
        one_match: ("ABC ABCDAB ABCDABCDABDE",  "ABCDABD", vec![15]),
        lots_of_matches: ("aaabaabaaaaa",  "aa", vec![0, 1, 4, 7, 8, 9, 10]),
        lots_of_intricate_matches: ("ababababa", "aba", vec![0, 2, 4, 6]),
        not_found0: ("abcde", "f", vec![]),
        not_found1: ("abcde", "ac", vec![]),
        not_found2: ("ababab", "bababa", vec![]),
        empty_string: ("", "abcdef", vec![]),
        empty_pattern: ("abcdef", "", vec![]),
        single_character_string: ("a", "a", vec![0]),
        single_character_pattern: ("abcdef", "d", vec![3]),
        pattern_at_start: ("abcdef", "abc", vec![0]),
        pattern_at_end: ("abcdef", "def", vec![3]),
        pattern_in_middle: ("abcdef", "cd", vec![2]),
        no_match_with_repeated_characters: ("aaaaaa", "b", vec![]),
        pattern_longer_than_string: ("abc", "abcd", vec![]),
        very_long_string: (&"a".repeat(10000), "a", (0..10000).collect::<Vec<usize>>()),
        very_long_pattern: (&"a".repeat(10000), &"a".repeat(9999), (0..2).collect::<Vec<usize>>()),
    }
}
