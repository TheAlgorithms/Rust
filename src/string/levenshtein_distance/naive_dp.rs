//! Provides functions to calculate the Levenshtein distance between two strings.
//!
//! The Levenshtein distance is a measure of the similarity between two strings by calculating the minimum number of single-character
//! edits (insertions, deletions, or substitutions) required to change one string into the other.

/// Calculates the Levenshtein distance between two strings.
///
/// # Arguments
///
/// * `string1` - A reference to the first string.
/// * `string2` - A reference to the second string.
///
/// # Returns
///
/// The Levenshtein distance between the two input strings.
pub fn naive_levenshtein_distance(string1: &str, string2: &str) -> usize {
    let distance_matrix: Vec<Vec<usize>> = (0..=string1.len())
        .map(|i| {
            (0..=string2.len())
                .map(|j| {
                    if i == 0 {
                        j
                    } else if j == 0 {
                        i
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();

    let updated_matrix = (1..=string1.len()).fold(distance_matrix, |matrix, i| {
        (1..=string2.len()).fold(matrix, |mut inner_matrix, j| {
            let cost = if string1.as_bytes()[i - 1] == string2.as_bytes()[j - 1] {
                0
            } else {
                1
            };
            inner_matrix[i][j] = (inner_matrix[i - 1][j - 1] + cost)
                .min(inner_matrix[i][j - 1] + 1)
                .min(inner_matrix[i - 1][j] + 1);
            inner_matrix
        })
    });

    updated_matrix[string1.len()][string2.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! naive_levenshtein_distance_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (string1, string2, expected_distance) = $test_case;
                    assert_eq!(
                        naive_levenshtein_distance(string1, string2),
                        expected_distance
                    );
                    assert_eq!(
                        naive_levenshtein_distance(string2, string1),
                        expected_distance
                    );
                    assert_eq!(
                        naive_levenshtein_distance(string1, string1),
                        0
                    );
                    assert_eq!(
                        naive_levenshtein_distance(string2, string2),
                        0
                    );
                }
            )*
        };
    }

    naive_levenshtein_distance_tests! {
        test_empty_strings: ("", "", 0),
        test_same_strings: ("Hello, World!", "Hello, World!", 0),
        test_one_empty_string: ("", "Rust", 4),
        test_longer_first_string: ("horse", "ros", 3),
        test_longer_second_string: ("execute", "intention", 8),
        test_different_strings: ("elephant", "ant", 5),
    }
}
