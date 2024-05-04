//! Provides functions to calculate the Levenshtein distance between two strings.
//!
//! The Levenshtein distance is a measure of the similarity between two strings by calculating the minimum number of single-character
//! edits (insertions, deletions, or substitutions) required to change one string into the other.

use std::cmp::min;

/// Calculates the Levenshtein distance between two strings using a naive dynamic programming approach.
///
/// The Levenshtein distance is a measure of the similarity between two strings by calculating the minimum number of single-character
/// edits (insertions, deletions, or substitutions) required to change one string into the other.
///
/// # Arguments
///
/// * `string1` - A reference to the first string.
/// * `string2` - A reference to the second string.
///
/// # Returns
///
/// The Levenshtein distance between the two input strings.
///
/// This function computes the Levenshtein distance by constructing a dynamic programming matrix and iteratively filling it in.
/// It follows the standard top-to-bottom, left-to-right approach for filling in the matrix.
///
/// # Complexity
///
/// - Time complexity: O(nm),
/// - Space complexity: O(nm),
///
/// where n and m are lengths of `string1` and `string2`.
///
/// Note that this implementation uses a straightforward dynamic programming approach without any space optimization.
/// It may consume more memory for larger input strings compared to the optimized version.
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

/// Calculates the Levenshtein distance between two strings using an optimized dynamic programming approach.
///
/// This edit distance is defined as 1 point per insertion, substitution, or deletion required to make the strings equal.
///
/// # Arguments
///
/// * `string1` - The first string.
/// * `string2` - The second string.
///
/// # Returns
///
/// The Levenshtein distance between the two input strings.
/// For a detailed explanation, check the example on [Wikipedia](https://en.wikipedia.org/wiki/Levenshtein_distance).
/// This function iterates over the bytes in the string, so it may not behave entirely as expected for non-ASCII strings.
///
/// Note that this implementation utilizes an optimized dynamic programming approach, significantly reducing the space complexity from O(nm) to O(n), where n and m are the lengths of `string1` and `string2`.
///
/// Additionally, it minimizes space usage by leveraging the shortest string horizontally and the longest string vertically in the computation matrix.
///
/// # Complexity
///
/// - Time complexity: O(nm),
/// - Space complexity: O(n),
///
/// where n and m are lengths of `string1` and `string2`.
pub fn optimized_levenshtein_distance(string1: &str, string2: &str) -> usize {
    if string1.is_empty() {
        return string2.len();
    }
    let l1 = string1.len();
    let mut prev_dist: Vec<usize> = (0..=l1).collect();

    for (row, c2) in string2.chars().enumerate() {
        // we'll keep a reference to matrix[i-1][j-1] (top-left cell)
        let mut prev_substitution_cost = prev_dist[0];
        // diff with empty string, since `row` starts at 0, it's `row + 1`
        prev_dist[0] = row + 1;

        for (col, c1) in string1.chars().enumerate() {
            // "on the left" in the matrix (i.e. the value we just computed)
            let deletion_cost = prev_dist[col] + 1;
            // "on the top" in the matrix (means previous)
            let insertion_cost = prev_dist[col + 1] + 1;
            let substitution_cost = if c1 == c2 {
                // last char is the same on both ends, so the min_distance is left unchanged from matrix[i-1][i+1]
                prev_substitution_cost
            } else {
                // substitute the last character
                prev_substitution_cost + 1
            };
            // save the old value at (i-1, j-1)
            prev_substitution_cost = prev_dist[col + 1];
            prev_dist[col + 1] = _min3(deletion_cost, insertion_cost, substitution_cost);
        }
    }
    prev_dist[l1]
}

#[inline]
fn _min3<T: Ord>(a: T, b: T, c: T) -> T {
    min(a, min(b, c))
}

#[cfg(test)]
mod tests {
    const LEVENSHTEIN_DISTANCE_TEST_CASES: &[(&str, &str, usize)] = &[
        ("", "", 0),
        ("Hello, World!", "Hello, World!", 0),
        ("", "Rust", 4),
        ("horse", "ros", 3),
        ("tan", "elephant", 6),
        ("execute", "intention", 8),
    ];

    macro_rules! levenshtein_distance_tests {
        ($function:ident) => {
            mod $function {
                use super::*;

                fn run_test_case(string1: &str, string2: &str, expected_distance: usize) {
                    assert_eq!(super::super::$function(string1, string2), expected_distance);
                    assert_eq!(super::super::$function(string2, string1), expected_distance);
                    assert_eq!(super::super::$function(string1, string1), 0);
                    assert_eq!(super::super::$function(string2, string2), 0);
                }

                #[test]
                fn test_levenshtein_distance() {
                    for &(string1, string2, expected_distance) in
                        LEVENSHTEIN_DISTANCE_TEST_CASES.iter()
                    {
                        run_test_case(string1, string2, expected_distance);
                    }
                }
            }
        };
    }

    levenshtein_distance_tests!(naive_levenshtein_distance);
    levenshtein_distance_tests!(optimized_levenshtein_distance);
}
