//! This module provides functionalities to match patterns in strings
//! and compute the Z-array for a given input string.

/// Calculates the Z-value for a given substring of the input string
/// based on a specified pattern.
///
/// # Parameters
/// - `input_string`: A slice of elements that represents the input string.
/// - `pattern`: A slice of elements representing the pattern to match.
/// - `start_index`: The index in the input string to start checking for matches.
/// - `z_value`: The initial Z-value to be computed.
///
/// # Returns
/// The computed Z-value indicating the length of the matching prefix.
fn calculate_z_value<T: Eq>(
    input_string: &[T],
    pattern: &[T],
    start_index: usize,
    mut z_value: usize,
) -> usize {
    let size = input_string.len();
    let pattern_size = pattern.len();

    while (start_index + z_value) < size && z_value < pattern_size {
        if input_string[start_index + z_value] != pattern[z_value] {
            break;
        }
        z_value += 1;
    }
    z_value
}

/// Initializes the Z-array value based on a previous match and updates
/// it to optimize further calculations.
///
/// # Parameters
/// - `z_array`: A mutable slice of the Z-array to be updated.
/// - `i`: The current index in the input string.
/// - `match_end`: The index of the last character matched in the pattern.
/// - `last_match`: The index of the last match found.
///
/// # Returns
/// The initialized Z-array value for the current index.
fn initialize_z_array_from_previous_match(
    z_array: &[usize],
    i: usize,
    match_end: usize,
    last_match: usize,
) -> usize {
    std::cmp::min(z_array[i - last_match], match_end - i + 1)
}

/// Finds the starting indices of all full matches of the pattern
/// in the Z-array.
///
/// # Parameters
/// - `z_array`: A slice of the Z-array containing computed Z-values.
/// - `pattern_size`: The length of the pattern to find in the Z-array.
///
/// # Returns
/// A vector containing the starting indices of full matches.
fn find_full_matches(z_array: &[usize], pattern_size: usize) -> Vec<usize> {
    z_array
        .iter()
        .enumerate()
        .filter_map(|(idx, &z_value)| (z_value == pattern_size).then_some(idx))
        .collect()
}

/// Matches the occurrences of a pattern in an input string starting
/// from a specified index.
///
/// # Parameters
/// - `input_string`: A slice of elements to search within.
/// - `pattern`: A slice of elements that represents the pattern to match.
/// - `start_index`: The index in the input string to start the search.
/// - `only_full_matches`: If true, only full matches of the pattern will be returned.
///
/// # Returns
/// A vector containing the starting indices of the matches.
fn match_with_z_array<T: Eq>(
    input_string: &[T],
    pattern: &[T],
    start_index: usize,
    only_full_matches: bool,
) -> Vec<usize> {
    let size = input_string.len();
    let pattern_size = pattern.len();
    let mut last_match: usize = 0;
    let mut match_end: usize = 0;
    let mut z_array = vec![0usize; size];

    for i in start_index..size {
        if i <= match_end {
            z_array[i] = initialize_z_array_from_previous_match(&z_array, i, match_end, last_match);
        }

        z_array[i] = calculate_z_value(input_string, pattern, i, z_array[i]);

        if i + z_array[i] > match_end + 1 {
            match_end = i + z_array[i] - 1;
            last_match = i;
        }
    }

    if !only_full_matches {
        z_array
    } else {
        find_full_matches(&z_array, pattern_size)
    }
}

/// Constructs the Z-array for the given input string.
///
/// The Z-array is an array where the i-th element is the length of the longest
/// substring starting from s[i] that is also a prefix of s.
///
/// # Parameters
/// - `input`: A slice of the input string for which the Z-array is to be constructed.
///
/// # Returns
/// A vector representing the Z-array of the input string.
pub fn z_array<T: Eq>(input: &[T]) -> Vec<usize> {
    match_with_z_array(input, input, 1, false)
}

/// Matches the occurrences of a given pattern in an input string.
///
/// This function acts as a wrapper around `match_with_z_array` to provide a simpler
/// interface for pattern matching, returning only full matches.
///
/// # Parameters
/// - `input`: A slice of the input string where the pattern will be searched.
/// - `pattern`: A slice of the pattern to search for in the input string.
///
/// # Returns
/// A vector of indices where the pattern matches the input string.
pub fn match_pattern<T: Eq>(input: &[T], pattern: &[T]) -> Vec<usize> {
    match_with_z_array(input, pattern, 0, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_match_pattern {
        ($($name:ident: ($input:expr, $pattern:expr, $expected:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, pattern, expected) = ($input, $pattern, $expected);
                    assert_eq!(match_pattern(input.as_bytes(), pattern.as_bytes()), expected);
                }
            )*
        };
    }

    macro_rules! test_z_array_cases {
        ($($name:ident: ($input:expr, $expected:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = ($input, $expected);
                    assert_eq!(z_array(input.as_bytes()), expected);
                }
            )*
        };
    }

    test_match_pattern! {
        simple_match: ("abcabcabc", "abc", vec![0, 3, 6]),
        no_match: ("abcdef", "xyz", vec![]),
        single_char_match: ("aaaaaa", "a", vec![0, 1, 2, 3, 4, 5]),
        overlapping_match: ("abababa", "aba", vec![0, 2, 4]),
        full_string_match: ("pattern", "pattern", vec![0]),
        empty_pattern: ("nonempty", " ", vec![]),
        pattern_larger_than_text: ("small", "largerpattern", vec![]),
        repeated_pattern_in_text: (
            "aaaaaaaa",
            "aaa",
            vec![0, 1, 2, 3, 4, 5]
        ),
        pattern_not_in_lipsum: (
            concat!(
                "lorem ipsum dolor sit amet, consectetur ",
                "adipiscing elit, sed do eiusmod tempor ",
                "incididunt ut labore et dolore magna aliqua"
            ),
            ";alksdjfoiwer",
            vec![]
        ),
        pattern_in_lipsum: (
            concat!(
                "lorem ipsum dolor sit amet, consectetur ",
                "adipiscing elit, sed do eiusmod tempor ",
                "incididunt ut labore et dolore magna aliqua"
            ),
            "m",
            vec![4, 10, 23, 68, 74, 110]
        ),
    }

    test_z_array_cases! {
        basic_z_array: ("aabaabab", vec![0, 1, 0, 4, 1, 0, 1, 0]),
        empty_string: ("", vec![]),
        single_char_z_array: ("a", vec![0]),
        repeated_char_z_array: ("aaaaaa", vec![0, 5, 4, 3, 2, 1]),
    }
}
