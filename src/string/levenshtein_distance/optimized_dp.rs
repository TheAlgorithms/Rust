//! Provides functions to calculate the Levenshtein distance between two strings.
//!
//! The Levenshtein distance is a measure of the similarity between two strings by calculating the minimum number of single-character
//! edits (insertions, deletions, or substitutions) required to change one string into the other.

use std::cmp::min;

/// Calculates the Levenshtein distance between two strings.
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
/// Note that although we compute a matrix, left-to-right, top-to-bottom, at each step all we need to compute `cell[i][j]` is:
///   - `cell[i][j-1]`
///   - `cell[i-j][j]`
///   - `cell[i-i][j-1]`
///
/// This can be achieved by only using one "rolling" row and one additional variable, when computed `cell[i][j]` (or `row[i]`):
///   - `cell[i][j-1]` is the value to the left, on the same row (the one we just computed, `row[i-1]`)
///   - `cell[i-1][j]` is the value at `row[i]`, the one we're changing
///   - `cell[i-1][j-1]` was the value at `row[i-1]` before we changed it, for that we'll use a variable
///
/// Doing this reduces space complexity from O(nm) to O(n)
///
/// Second note: if we want to minimize space, since we're now O(n) make sure you use the shortest string horizontally, and the longest vertically
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
        let mut prev_substitution_cost = prev_dist[0]; // we'll keep a reference to matrix[i-1][j-1] (top-left cell)
        prev_dist[0] = row + 1; // diff with empty string, since `row` starts at 0, it's `row + 1`

        for (col, c1) in string1.chars().enumerate() {
            let deletion_cost = prev_dist[col] + 1; // "on the left" in the matrix (i.e. the value we just computed)
            let insertion_cost = prev_dist[col + 1] + 1; // "on the top" in the matrix (means previous)
            let substitution_cost = if c1 == c2 {
                prev_substitution_cost // last char is the same on both ends, so the min_distance is left unchanged from matrix[i-1][i+1]
            } else {
                prev_substitution_cost + 1 // substitute the last character
            };

            prev_substitution_cost = prev_dist[col + 1]; // save the old value at (i-1, j-1)
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
    use super::_min3;
    use super::optimized_levenshtein_distance;

    #[test]
    fn test_doc_example() {
        assert_eq!(2, optimized_levenshtein_distance("FROG", "DOG"));
    }

    #[test]
    fn return_0_with_empty_strings() {
        assert_eq!(0, optimized_levenshtein_distance("", ""));
    }

    #[test]
    fn return_1_with_empty_and_a() {
        assert_eq!(1, optimized_levenshtein_distance("", "a"));
    }

    #[test]
    fn return_1_with_a_and_empty() {
        assert_eq!(1, optimized_levenshtein_distance("a", ""));
    }

    #[test]
    fn return_1_with_ab_and_a() {
        assert_eq!(1, optimized_levenshtein_distance("ab", "a"));
    }

    #[test]
    fn return_0_with_foobar_and_foobar() {
        assert_eq!(0, optimized_levenshtein_distance("foobar", "foobar"));
    }

    #[test]
    fn return_6_with_foobar_and_barfoo() {
        assert_eq!(6, optimized_levenshtein_distance("foobar", "barfoo"));
    }

    #[test]
    fn return_1_with_kind_and_bind() {
        assert_eq!(1, optimized_levenshtein_distance("kind", "bind"));
    }

    #[test]
    fn return_3_with_winner_and_win() {
        assert_eq!(3, optimized_levenshtein_distance("winner", "win"));
    }

    #[test]
    fn equal_strings() {
        assert_eq!(
            0,
            optimized_levenshtein_distance("Hello, world!", "Hello, world!")
        );
        assert_eq!(
            0,
            optimized_levenshtein_distance("Hello, world!", "Hello, world!")
        );
        assert_eq!(
            0,
            optimized_levenshtein_distance("Test_Case_#1", "Test_Case_#1")
        );
        assert_eq!(
            0,
            optimized_levenshtein_distance("Test_Case_#1", "Test_Case_#1")
        );
    }

    #[test]
    fn one_edit_difference() {
        assert_eq!(
            1,
            optimized_levenshtein_distance("Hello, world!", "Hell, world!")
        );
        assert_eq!(
            1,
            optimized_levenshtein_distance("Test_Case_#1", "Test_Case_#2")
        );
        assert_eq!(
            1,
            optimized_levenshtein_distance("Test_Case_#1", "Test_Case_#10")
        );
        assert_eq!(
            1,
            optimized_levenshtein_distance("Hello, world!", "Hell, world!")
        );
        assert_eq!(
            1,
            optimized_levenshtein_distance("Test_Case_#1", "Test_Case_#2")
        );
        assert_eq!(
            1,
            optimized_levenshtein_distance("Test_Case_#1", "Test_Case_#10")
        );
    }

    #[test]
    fn several_differences() {
        assert_eq!(2, optimized_levenshtein_distance("My Cat", "My Case"));
        assert_eq!(
            7,
            optimized_levenshtein_distance("Hello, world!", "Goodbye, world!")
        );
        assert_eq!(6, optimized_levenshtein_distance("Test_Case_#3", "Case #3"));
        assert_eq!(2, optimized_levenshtein_distance("My Cat", "My Case"));
        assert_eq!(
            7,
            optimized_levenshtein_distance("Hello, world!", "Goodbye, world!")
        );
        assert_eq!(6, optimized_levenshtein_distance("Test_Case_#3", "Case #3"));
    }

    #[test]
    fn return_1_with_1_2_3() {
        assert_eq!(1, _min3(1, 2, 3));
    }

    #[test]
    fn return_1_with_3_2_1() {
        assert_eq!(1, _min3(3, 2, 1));
    }

    #[test]
    fn return_1_with_2_3_1() {
        assert_eq!(1, _min3(2, 3, 1));
    }
}
