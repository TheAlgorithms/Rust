//! Compute the edit distance between two strings

use std::cmp::min;

/// edit_distance(str_a, str_b) returns the edit distance between the two
/// strings This edit distance is defined as being 1 point per insertion,
/// substitution, or deletion which must be made to make the strings equal.
///
/// This function iterates over the bytes in the string, so it may not behave
/// entirely as expected for non-ASCII strings.
///
/// # Complexity
///
/// - time complexity: O(nm),
/// - space complexity: O(nm),
///
/// where n and m are lengths of `str_a` and `str_b`
pub fn edit_distance(str_a: &str, str_b: &str) -> u32 {
    // distances[i][j] = distance between a[..i] and b[..j]
    let mut distances = vec![vec![0; str_b.len() + 1]; str_a.len() + 1];
    // Initialize cases in which one string is empty
    for j in 0..=str_b.len() {
        distances[0][j] = j as u32;
    }
    for (i, item) in distances.iter_mut().enumerate() {
        item[0] = i as u32;
    }
    for i in 1..=str_a.len() {
        for j in 1..=str_b.len() {
            distances[i][j] = min(distances[i - 1][j] + 1, distances[i][j - 1] + 1);
            if str_a.as_bytes()[i - 1] == str_b.as_bytes()[j - 1] {
                distances[i][j] = min(distances[i][j], distances[i - 1][j - 1]);
            } else {
                distances[i][j] = min(distances[i][j], distances[i - 1][j - 1] + 1);
            }
        }
    }
    distances[str_a.len()][str_b.len()]
}

/// The space efficient version of the above algorithm.
///
/// Instead of storing the `m * n` matrix expicitly, only one row (of length `n`) is stored.
/// It keeps overwriting itself based on its previous values with the help of two scalars,
/// gradually reaching the last row. Then, the score is `matrix[n]`.
///
/// # Complexity
///
/// - time complexity: O(nm),
/// - space complexity: O(n),
///
/// where n and m are lengths of `str_a` and `str_b`
pub fn edit_distance_se(str_a: &str, str_b: &str) -> u32 {
    let (str_a, str_b) = (str_a.as_bytes(), str_b.as_bytes());
    let (m, n) = (str_a.len(), str_b.len());
    let mut distances: Vec<u32> = vec![0; n + 1]; // the dynamic programming matrix (only 1 row stored)
    let mut s: u32; // distances[i - 1][j - 1] or distances[i - 1][j]
    let mut c: u32; // distances[i][j - 1] or distances[i][j]
    let mut a: u8; // str_a[i - 1] the i-th character in str_a; only needs to be computed once per row
    let mut b: u8; // str_b[j - 1] the j-th character in str_b

    // 0th row
    for (j, v) in distances.iter_mut().enumerate().take(n + 1).skip(1) {
        *v = j as u32;
    }
    // rows 1 to m
    for i in 1..=m {
        s = (i - 1) as u32;
        c = i as u32;
        a = str_a[i - 1];
        for j in 1..=n {
            // c is distances[i][j-1] and s is distances[i-1][j-1] at the beginning of each round of iteration
            b = str_b[j - 1];
            c = min(s + if a == b { 0 } else { 1 }, min(c + 1, distances[j] + 1));
            // c is updated to distances[i][j], and will thus become distances[i][j-1] for the next cell
            s = distances[j]; // here distances[j] means distances[i-1][j] becuase it has not been overwritten yet
                              // s is updated to distances[i-1][j], and will thus become distances[i-1][j-1] for the next cell
            distances[j] = c; // now distances[j] is updated to distances[i][j], and will thus become distances[i-1][j] for the next ROW
        }
    }

    distances[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_strings() {
        assert_eq!(0, edit_distance("Hello, world!", "Hello, world!"));
        assert_eq!(0, edit_distance_se("Hello, world!", "Hello, world!"));
        assert_eq!(0, edit_distance("Test_Case_#1", "Test_Case_#1"));
        assert_eq!(0, edit_distance_se("Test_Case_#1", "Test_Case_#1"));
    }

    #[test]
    fn one_edit_difference() {
        assert_eq!(1, edit_distance("Hello, world!", "Hell, world!"));
        assert_eq!(1, edit_distance("Test_Case_#1", "Test_Case_#2"));
        assert_eq!(1, edit_distance("Test_Case_#1", "Test_Case_#10"));
        assert_eq!(1, edit_distance_se("Hello, world!", "Hell, world!"));
        assert_eq!(1, edit_distance_se("Test_Case_#1", "Test_Case_#2"));
        assert_eq!(1, edit_distance_se("Test_Case_#1", "Test_Case_#10"));
    }

    #[test]
    fn several_differences() {
        assert_eq!(2, edit_distance("My Cat", "My Case"));
        assert_eq!(7, edit_distance("Hello, world!", "Goodbye, world!"));
        assert_eq!(6, edit_distance("Test_Case_#3", "Case #3"));
        assert_eq!(2, edit_distance_se("My Cat", "My Case"));
        assert_eq!(7, edit_distance_se("Hello, world!", "Goodbye, world!"));
        assert_eq!(6, edit_distance_se("Test_Case_#3", "Case #3"));
    }
}
